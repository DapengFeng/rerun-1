// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/class_description.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: The description of a semantic Class.
///
/// If an entity is annotated with a corresponding [`components::ClassId`][crate::components::ClassId], Rerun will use
/// the attached [`datatypes::AnnotationInfo`][crate::datatypes::AnnotationInfo] to derive labels and colors.
///
/// Keypoints within an annotation class can similarly be annotated with a
/// [`components::KeypointId`][crate::components::KeypointId] in which case we should defer to the label and color for the
/// [`datatypes::AnnotationInfo`][crate::datatypes::AnnotationInfo] specifically associated with the Keypoint.
///
/// Keypoints within the class can also be decorated with skeletal edges.
/// Keypoint-connections are pairs of [`components::KeypointId`][crate::components::KeypointId]s. If an edge is
/// defined, and both keypoints exist within the instance of the class, then the
/// keypoints should be connected with an edge. The edge should be labeled and
/// colored as described by the class's [`datatypes::AnnotationInfo`][crate::datatypes::AnnotationInfo].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassDescription {
    /// The [`datatypes::AnnotationInfo`][crate::datatypes::AnnotationInfo] for the class.
    pub info: crate::datatypes::AnnotationInfo,

    /// The [`datatypes::AnnotationInfo`][crate::datatypes::AnnotationInfo] for all of the keypoints.
    pub keypoint_annotations: Vec<crate::datatypes::AnnotationInfo>,

    /// The connections between keypoints.
    pub keypoint_connections: Vec<crate::datatypes::KeypointPair>,
}

impl ::re_types_core::SizeBytes for ClassDescription {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.info.heap_size_bytes()
            + self.keypoint_annotations.heap_size_bytes()
            + self.keypoint_connections.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::AnnotationInfo>::is_pod()
            && <Vec<crate::datatypes::AnnotationInfo>>::is_pod()
            && <Vec<crate::datatypes::KeypointPair>>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(ClassDescription);

impl ::re_types_core::Loggable for ClassDescription {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.ClassDescription".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new(
                "info",
                <crate::datatypes::AnnotationInfo>::arrow_datatype(),
                false,
            ),
            Field::new(
                "keypoint_annotations",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    <crate::datatypes::AnnotationInfo>::arrow_datatype(),
                    false,
                ))),
                false,
            ),
            Field::new(
                "keypoint_connections",
                DataType::List(std::sync::Arc::new(Field::new(
                    "item",
                    <crate::datatypes::KeypointPair>::arrow_datatype(),
                    false,
                ))),
                false,
            ),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, info): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.info.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let info_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = info_bitmap;
                            crate::datatypes::AnnotationInfo::to_arrow_opt(info)?
                        }
                    },
                    {
                        let (somes, keypoint_annotations): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| datum.keypoint_annotations.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint_annotations_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                                keypoint_annotations
                                    .iter()
                                    .map(|opt| opt.as_ref().map_or(0, |datum| datum.len())),
                            )?
                            .into();
                            let keypoint_annotations_inner_data: Vec<_> = keypoint_annotations
                                .into_iter()
                                .flatten()
                                .flatten()
                                .collect();
                            let keypoint_annotations_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                                None;
                            ListArray::try_new(
                                DataType::List(std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::AnnotationInfo>::arrow_datatype(),
                                    false,
                                ))),
                                offsets,
                                {
                                    _ = keypoint_annotations_inner_bitmap;
                                    crate::datatypes::AnnotationInfo::to_arrow_opt(
                                        keypoint_annotations_inner_data.into_iter().map(Some),
                                    )?
                                },
                                keypoint_annotations_bitmap,
                            )?
                            .boxed()
                        }
                    },
                    {
                        let (somes, keypoint_connections): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| datum.keypoint_connections.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint_connections_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                                keypoint_connections
                                    .iter()
                                    .map(|opt| opt.as_ref().map_or(0, |datum| datum.len())),
                            )?
                            .into();
                            let keypoint_connections_inner_data: Vec<_> = keypoint_connections
                                .into_iter()
                                .flatten()
                                .flatten()
                                .collect();
                            let keypoint_connections_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                                None;
                            ListArray::try_new(
                                DataType::List(std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::KeypointPair>::arrow_datatype(),
                                    false,
                                ))),
                                offsets,
                                {
                                    _ = keypoint_connections_inner_bitmap;
                                    crate::datatypes::KeypointPair::to_arrow_opt(
                                        keypoint_connections_inner_data.into_iter().map(Some),
                                    )?
                                },
                                keypoint_connections_bitmap,
                            )?
                            .boxed()
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.ClassDescription")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let info = {
                    if !arrays_by_name.contains_key("info") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "info",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let arrow_data = &**arrays_by_name["info"];
                    crate::datatypes::AnnotationInfo::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ClassDescription#info")?
                        .into_iter()
                };
                let keypoint_annotations = {
                    if !arrays_by_name.contains_key("keypoint_annotations") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "keypoint_annotations",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let arrow_data = &**arrays_by_name["keypoint_annotations"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::List(std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::AnnotationInfo>::arrow_datatype(),
                                    false,
                                )));
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.datatypes.ClassDescription#keypoint_annotations",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::datatypes::AnnotationInfo::from_arrow_opt(arrow_data_inner)
                                    .with_context(
                                        "rerun.datatypes.ClassDescription#keypoint_annotations",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data =
                                        unsafe { arrow_data_inner.get_unchecked(start..end) };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let keypoint_connections = {
                    if !arrays_by_name.contains_key("keypoint_connections") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "keypoint_connections",
                        ))
                        .with_context("rerun.datatypes.ClassDescription");
                    }
                    let arrow_data = &**arrays_by_name["keypoint_connections"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::List(std::sync::Arc::new(Field::new(
                                    "item",
                                    <crate::datatypes::KeypointPair>::arrow_datatype(),
                                    false,
                                )));
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.datatypes.ClassDescription#keypoint_connections",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::datatypes::KeypointPair::from_arrow_opt(arrow_data_inner)
                                    .with_context(
                                        "rerun.datatypes.ClassDescription#keypoint_connections",
                                    )?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data =
                                        unsafe { arrow_data_inner.get_unchecked(start..end) };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(info, keypoint_annotations, keypoint_connections),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(info, keypoint_annotations, keypoint_connections)| {
                        Ok(Self {
                            info: info
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.ClassDescription#info")?,
                            keypoint_annotations: keypoint_annotations
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescription#keypoint_annotations",
                                )?,
                            keypoint_connections: keypoint_connections
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescription#keypoint_connections",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.ClassDescription")?
            }
        })
    }
}
