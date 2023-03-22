use std::{num::NonZeroU32, sync::Arc};

use ahash::{HashMap, HashSet};

use crate::{
    wgpu_resources::{GpuTexture, GpuTexturePool, TextureDesc},
    DebugLabel,
};

use super::ResourceManagerError;

/// Handle to a 2D resource.
///
/// Currently, this is solely a more strongly typed regular gpu texture handle.
/// Since all textures have "long lived" behavior (no temp allocation, alive until unused),
/// there is no difference as with buffer reliant data like meshes or most contents of draw-data.
#[derive(Clone)]
pub struct GpuTexture2DHandle(Option<GpuTexture>);

impl GpuTexture2DHandle {
    pub fn invalid() -> Self {
        Self(None)
    }
}

/// Data required to create a texture 2d resource.
///
/// It is *not* stored along side the resulting texture resource!
pub struct Texture2DCreationDesc<'a> {
    pub label: DebugLabel,

    /// Data for the highest mipmap level.
    /// Must be padded according to wgpu rules and ready for upload.
    /// TODO(andreas): This should be a kind of factory function/builder instead which gets target memory passed in.
    pub data: &'a [u8],
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
    //generate_mip_maps: bool, // TODO(andreas): generate mipmaps!
}

impl<'a> Texture2DCreationDesc<'a> {
    pub fn convert_rgb8_to_rgba8(rgb_pixels: &[u8]) -> Vec<u8> {
        rgb_pixels
            .chunks_exact(3)
            .flat_map(|color| [color[0], color[1], color[2], 255])
            .collect()
    }
}

/// Texture manager for 2D textures.
///
/// The scope is intentionally limited to particular kinds of textures that currently
/// require this kind of handle abstraction/management.
/// More complex textures types are typically handled within renderer which utilize the texture pool directly.
/// This manager in contrast, deals with user provided texture data!
/// We might revisit this later and make this texture manager more general purpose.
pub struct TextureManager2D {
    // Long lived/short lived doesn't make sense for textures since we don't yet know a way to
    // optimize for short lived textures as we do with buffer data.
    //manager: ResourceManager<Texture2DHandleInner, GpuTextureHandleStrong>,
    white_texture: GpuTexture2DHandle,

    // For convenience to reduce amount of times we need to pass them around
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    // Cache the texture using a user-provided u64 id. This is expected
    // to be derived from the `TensorId` which isn't available here for
    // dependency reasons.
    // TODO(jleibs): Introduce a proper key here.
    //
    // Any texture which wasn't accessed on the previous frame
    // is ejected from the cache during [`begin_frame`].
    texture_cache: HashMap<u64, GpuTexture2DHandle>,
    accessed_textures: HashSet<u64>,
}

impl TextureManager2D {
    pub(crate) fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        texture_pool: &mut GpuTexturePool,
    ) -> Self {
        let white_texture = Self::create_and_upload_texture(
            &device,
            &queue,
            texture_pool,
            &Texture2DCreationDesc {
                label: "placeholder".into(),
                data: &[255, 255, 255, 255],
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                width: 1,
                height: 1,
            },
        );

        Self {
            white_texture,
            device,
            queue,
            texture_cache: Default::default(),
            accessed_textures: Default::default(),
        }
    }

    /// Creates a new 2D texture resource and schedules data upload to the GPU.
    /// TODO(jleibs): All usages of this should be be replaced with `get_or_create`, which is strictly preferable
    pub fn create(
        &mut self,
        texture_pool: &mut GpuTexturePool,
        creation_desc: &Texture2DCreationDesc<'_>,
    ) -> GpuTexture2DHandle {
        // TODO(andreas): Disabled the warning as we're moving towards using this texture manager for user-logged images.
        // However, it's still very much a concern especially once we add mipmapping. Something we need to keep in mind.
        //
        // if !resource.width.is_power_of_two() || !resource.width.is_power_of_two() {
        //     re_log::warn!(
        //         "Texture {:?} has the non-power-of-two (NPOT) resolution of {}x{}. \
        //         NPOT textures are slower and on WebGL can't handle mipmapping, UV wrapping and UV tiling",
        //         resource.label,
        //         resource.width,
        //         resource.height
        //     );
        // }

        // Currently we don't store any data in the the texture manager.
        // In the future we might handle (lazy?) mipmap generation in here or keep track of lazy upload processing.

        Self::create_and_upload_texture(&self.device, &self.queue, texture_pool, creation_desc)
    }

    /// Creates a new 2D texture resource and schedules data upload to the GPU if a texture
    /// wasn't already created using the same key.
    pub fn get_or_create(
        &mut self,
        key: u64,
        texture_pool: &mut GpuTexturePool,
        creation_desc: &Texture2DCreationDesc<'_>,
    ) -> GpuTexture2DHandle {
        let texture_handle = self.texture_cache.entry(key).or_insert_with(|| {
            Self::create_and_upload_texture(&self.device, &self.queue, texture_pool, creation_desc)
        });
        self.accessed_textures.insert(key);
        texture_handle.clone()
    }

    /// Returns a single pixel white pixel.
    pub fn white_texture_handle(&self) -> &GpuTexture2DHandle {
        &self.white_texture
    }

    fn create_and_upload_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_pool: &mut GpuTexturePool,
        creation_desc: &Texture2DCreationDesc<'_>,
    ) -> GpuTexture2DHandle {
        crate::profile_function!();
        let size = wgpu::Extent3d {
            width: creation_desc.width,
            height: creation_desc.height,
            depth_or_array_layers: 1,
        };
        let texture = texture_pool.alloc(
            device,
            &TextureDesc {
                label: creation_desc.label.clone(),
                size,
                mip_level_count: 1, // TODO(andreas)
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: creation_desc.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
        );

        let format_info = creation_desc.format.describe();
        let width_blocks = creation_desc.width / format_info.block_dimensions.0 as u32;
        let bytes_per_row_unaligned = width_blocks * format_info.block_size as u32;

        // TODO(andreas): Once we have our own temp buffer for uploading, we can do the padding inplace
        // I.e. the only difference will be if we do one memcopy or one memcopy per row, making row padding a nuisance!
        let data = creation_desc.data;

        // TODO(andreas): temp allocator for staging data?
        // We don't do any further validation of the buffer here as wgpu does so extensively.
        crate::profile_scope!("write_texture");
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(
                    NonZeroU32::new(bytes_per_row_unaligned).expect("invalid bytes per row"),
                ),
                rows_per_image: None,
            },
            size,
        );

        // TODO(andreas): mipmap generation

        GpuTexture2DHandle(Some(texture))
    }

    /// Retrieves gpu handle.
    #[allow(clippy::unused_self)]
    pub(crate) fn get(
        &self,
        handle: &GpuTexture2DHandle,
    ) -> Result<GpuTexture, ResourceManagerError> {
        handle
            .0
            .as_ref()
            .ok_or(ResourceManagerError::NullHandle)
            .map(|h| h.clone())
    }

    pub(crate) fn begin_frame(&mut self, _frame_index: u64) {
        // Drop any textures that weren't accessed in the last frame
        self.texture_cache
            .retain(|k, _| self.accessed_textures.contains(k));
        self.accessed_textures.clear();
    }
}
