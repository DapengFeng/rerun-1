use super::RotationAxisAngle;

use crate::datatypes::{Angle, Vec3D};

impl RotationAxisAngle {
    /// Identity rotation, representing no rotation.
    pub const IDENTITY: Self = Self {
        axis: Vec3D::new(1.0, 0.0, 0.0),
        angle: Angle::ZERO,
    };

    /// Create a new rotation from an axis and an angle.
    #[inline]
    pub fn new(axis: impl Into<Vec3D>, angle: impl Into<Angle>) -> Self {
        Self {
            axis: axis.into(),
            angle: angle.into(),
        }
    }
}

impl<V: Into<Vec3D>, A: Into<Angle>> From<(V, A)> for RotationAxisAngle {
    fn from((axis, angle): (V, A)) -> Self {
        Self::new(axis, angle)
    }
}

#[cfg(feature = "glam")]
impl From<RotationAxisAngle> for glam::Quat {
    #[inline]
    fn from(val: RotationAxisAngle) -> Self {
        let axis: glam::Vec3 = val.axis.into();
        axis.try_normalize()
            .map(|axis| Self::from_axis_angle(axis, val.angle.radians()))
            .unwrap_or_default()
    }
}

#[cfg(feature = "mint")]
impl From<RotationAxisAngle> for mint::Quaternion<f32> {
    #[inline]
    fn from(val: RotationAxisAngle) -> Self {
        let (s, c) = (val.angle.radians() * 0.5).sin_cos();
        [val.axis.x() * s, val.axis.y() * s, val.axis.z() * s, c].into()
    }
}

impl Default for RotationAxisAngle {
    #[inline]
    fn default() -> Self {
        Self {
            axis: Vec3D::new(1.0, 0.0, 0.0),
            angle: Angle::from_radians(0.0),
        }
    }
}
