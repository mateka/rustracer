use auto_ops::*;
use nalgebra::Unit;

use crate::{
    Isometry3, Matrix3, Matrix4, Point2, Point3, Rotation3, Scalar, Similarity3, Transform3,
    Translation3, Vector3,
};

/// 3D ray class for ray tracing.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Unit<Vector3>,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: Unit::new_normalize(direction),
        }
    }
}

/// Trait for all types, which are ray traceable in 3D
pub trait RayTraceable {
    /// Returns normal vector to the traceable object
    fn get_normal(&self) -> Unit<Vector3>;

    /// Returns size of the traceable object
    fn get_size(&self) -> Scalar;

    /// Computes point of intersection of Self with ray.
    /// If there is no intersection, it returns None.
    fn intersects(&self, ray: &Ray) -> Option<Point3>;

    /// Computes 2D local coordinates of 3D point inside ray traceable primitive.
    /// It can be used for example as a texture coordinates.
    fn local_2d_coordinates(&self, point: &Point3) -> Point2;
}

impl_op_ex!(*|a: &Matrix3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: Unit::new_normalize(a * b.direction.into_inner()),
    }
});

impl_op_ex!(*|a: &Rotation3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: Unit::new_normalize(a * b.direction.into_inner()),
    }
});

impl_op_ex!(*|a: &Translation3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: b.direction,
    }
});

impl_op_ex!(*|a: &Isometry3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: Unit::new_normalize(a * b.direction.into_inner()),
    }
});

impl_op_ex!(
    *|a: &nalgebra::Isometry<Scalar, nalgebra::U3, Rotation3>, b: &Ray| -> Ray {
        Ray {
            origin: a * b.origin,
            direction: Unit::new_normalize(a * b.direction.into_inner()),
        }
    }
);

impl_op_ex!(*|a: &Similarity3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: Unit::new_normalize(a * b.direction.into_inner()),
    }
});

impl_op_ex!(*|a: &Transform3, b: &Ray| -> Ray {
    Ray {
        origin: a * b.origin,
        direction: Unit::new_normalize(a * b.direction.into_inner()),
    }
});

impl_op_ex!(*|a: &Matrix4, b: &Ray| -> Ray {
    Ray {
        origin: a.transform_point(&b.origin),
        direction: Unit::new_normalize(a.transform_vector(&b.direction)),
    }
});

#[allow(clippy::op_ref)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quaternion;

    #[test]
    fn ray_can_be_multiplied_by_3d_matrix() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        #[rustfmt::skip]
        let mat = Matrix3::new(
            1.0, 2.0, 3.0,
            3.0, 1.5, -7.0,
            -std::f32::consts::PI, 1.57, -3.0
        );
        let expected = Ray {
            origin: mat * origin,
            direction: Unit::new_normalize(mat * direction.into_inner()),
        };

        assert_eq!(mat * ray, expected);
        assert_eq!(&mat * ray, expected);
        assert_eq!(mat * &ray, expected);
        assert_eq!(&mat * &ray, expected);
    }

    #[test]
    fn ray_can_be_rotated() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        let rotation = Rotation3::new(Vector3::new(1.57, 0.0, -0.75));
        let expected = Ray {
            origin: rotation * origin,
            direction: Unit::new_normalize(rotation * direction.into_inner()),
        };

        assert_eq!(rotation * ray, expected);
        assert_eq!(&rotation * ray, expected);
        assert_eq!(rotation * &ray, expected);
        assert_eq!(&rotation * &ray, expected);
    }

    #[test]
    fn ray_can_be_translated() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let expected = Ray {
            origin: translation * origin,
            direction,
        };

        assert_eq!(translation * ray, expected);
        assert_eq!(&translation * ray, expected);
        assert_eq!(translation * &ray, expected);
        assert_eq!(&translation * &ray, expected);
    }

    #[test]
    fn ray_can_be_rotated_and_translated_by_two_transforms() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        let rotation = Rotation3::new(Vector3::new(1.57, 0.0, -0.75));
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let expected = Ray {
            origin: rotation * translation * origin,
            direction: Unit::new_normalize(rotation * direction.into_inner()),
        };

        assert_eq!(rotation * translation * ray, expected);
        assert_eq!(rotation * translation * &ray, expected);
    }

    #[test]
    fn ray_can_be_rotated_and_translated() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        let isometry = Isometry3::new(Vector3::new(-1.0, 2.5, 0.0), Vector3::new(1.57, 0.0, -0.75));
        let expected = Ray {
            origin: isometry * origin,
            direction: Unit::new_normalize(isometry * direction.into_inner()),
        };

        assert_eq!(isometry * ray, expected);
        assert_eq!(&isometry * ray, expected);
        assert_eq!(isometry * &ray, expected);
        assert_eq!(&isometry * &ray, expected);
    }

    #[test]
    fn ray_can_be_transformed_into_similar_ray() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let rotation = Unit::new_normalize(Quaternion::new(1.75, 0.0, 1.0, 2.0));
        let similarity = Similarity3::from_parts(translation, rotation, 2.0);
        let expected = Ray {
            origin: similarity * origin,
            direction: Unit::new_normalize(similarity * direction.into_inner()),
        };

        assert_eq!(similarity * ray, expected);
        assert_eq!(&similarity * ray, expected);
        assert_eq!(similarity * &ray, expected);
        assert_eq!(&similarity * &ray, expected);
    }

    #[test]
    fn ray_can_be_transformed() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        #[rustfmt::skip]
        let transform = Transform3::from_matrix_unchecked(Matrix4::new(
            1.0, 2.0, 3.0, 0.0,
            3.0, 1.5, -7.0, 0.0,
            -std::f32::consts::PI, 1.57, -3.0, 0.0,
            0.0, 1.57, 0.0, 1.0,
        ));
        let expected = Ray {
            origin: transform * origin,
            direction: Unit::new_normalize(transform * direction.into_inner()),
        };

        assert_eq!(transform * ray, expected);
        assert_eq!(&transform * ray, expected);
        assert_eq!(transform * &ray, expected);
        assert_eq!(&transform * &ray, expected);
    }

    #[test]
    fn ray_can_be_multiplied_by_4d_matrix() {
        let origin = Point3::new(1.0, 2.0, -7.0);
        let direction = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0));
        let ray = Ray { origin, direction };
        #[rustfmt::skip]
        let matrix = Matrix4::new_scaling(2.0);
        let expected = Ray {
            origin: matrix.transform_point(&origin),
            direction: Unit::new_normalize(matrix.transform_vector(&direction)),
        };

        assert_eq!(matrix * ray, expected);
        assert_eq!(&matrix * ray, expected);
        assert_eq!(matrix * &ray, expected);
        assert_eq!(&matrix * &ray, expected);
    }
}
