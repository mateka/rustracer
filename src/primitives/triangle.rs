use image::Rgb;
use impl_ops::*;
use nalgebra::Unit;
use std::ops;

use crate::{
    ray::RayTraceable, Isometry3, Matrix3, Matrix4, Point3, Ray, Rotation3, Similarity3,
    Transform3, Translation3, Vector3,
};

/// A triangle primitive
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    vertices: [Point3; 3],
    normal: Unit<Vector3>,
    pub colour: Rgb<u8>, // TODO: Remove
}

impl Triangle {
    pub fn new(vertices: [Point3; 3], colour: Rgb<u8>) -> Triangle {
        Triangle {
            vertices: vertices,
            normal: Triangle::calculate_normal(vertices),
            colour: colour,
        }
    }

    pub fn new_triangle_colour(t: &Triangle, colour: Rgb<u8>) -> Triangle {
        Triangle {
            vertices: t.vertices,
            normal: t.normal,
            colour: colour,
        }
    }

    pub fn get_v(&self, index: usize) -> &Point3 {
        &self.vertices[index]
    }

    pub fn get_normal(&self) -> &Unit<Vector3> {
        &self.normal
    }

    pub fn set_v(&mut self, index: usize, value: Point3) {
        self.vertices[index] = value;
        self.normal = Triangle::calculate_normal(self.vertices);
    }

    fn calculate_normal(vertices: [Point3; 3]) -> Unit<Vector3> {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        Unit::new_normalize(e1.cross(&e2))
    }

    fn is_inside(&self, point: &Point3) -> bool {
        let edge1 = self.get_v(1) - self.get_v(0);
        let edge2 = self.get_v(2) - self.get_v(1);
        let edge3 = self.get_v(0) - self.get_v(2);
        let v1 = point - self.get_v(0);
        let v2 = point - self.get_v(1);
        let v3 = point - self.get_v(2);

        let norm = self.normal;
        norm.dot(&edge1.cross(&v1)) > 0.0
            && norm.dot(&edge2.cross(&v2)) > 0.0
            && norm.dot(&edge3.cross(&v3)) > 0.0
    }
}

impl RayTraceable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Point3> {
        let norm = self.normal;
        if 0.0 == norm.dot(&ray.direction) {
            return None;
        }
        let a_vector = self.get_v(0) - Point3::origin();
        let distance_to_origin = norm.dot(&a_vector);
        let origin_vector = ray.origin - Point3::origin();
        let t = -(norm.dot(&origin_vector) + distance_to_origin) / norm.dot(&ray.direction);
        if t < 0.0 {
            return None;
        }
        let plane_point = ray.origin + t * ray.direction.into_inner();
        if self.is_inside(&plane_point) {
            return Some(plane_point);
        }
        None
    }
}

impl_op_ex!(*|a: &Matrix3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Rotation3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Translation3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Isometry3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Similarity3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Transform3, b: &Triangle| -> Triangle {
    Triangle::new(
        [a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]],
        b.colour,
    )
});

impl_op_ex!(*|a: &Matrix4, b: &Triangle| -> Triangle {
    Triangle::new(
        [
            a.transform_point(&b.vertices[0]),
            a.transform_point(&b.vertices[1]),
            a.transform_point(&b.vertices[2]),
        ],
        b.colour,
    )
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quaternion;

    #[test]
    fn triangle_creation_normal_is_computed() {
        let tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );

        assert_eq!(tri.get_v(0), &Point3::new(0.0, 1.0, 0.0));
        assert_eq!(tri.get_v(1), &Point3::new(-0.5, 0.0, 0.0));
        assert_eq!(tri.get_v(2), &Point3::new(0.5, 0.0, 0.0));
        assert_eq!(
            tri.get_normal(),
            &Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0))
        );
        assert_eq!(tri.colour, Rgb([255u8, 125u8, 12u8]));
    }

    #[test]
    fn triangle_colouring_creation() {
        let source_tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );
        let expected_tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([0u8, 125u8, 255u8]),
        );
        assert_eq!(
            Triangle::new_triangle_colour(&source_tri, Rgb([0u8, 125u8, 255u8])),
            expected_tri
        );
    }

    #[test]
    fn triangle_does_not_intersect_parallel_ray() {
        let tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );
        let ray = Ray::new(Point3::new(-1.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_does_not_intersect_ray_starting_behind_triangle() {
        let tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );
        let ray = Ray::new(Point3::new(0.0, 0.5, 1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_does_not_intersect_ray_hitting_triangle_plane_but_not_the_triangle() {
        let tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );
        let ray = Ray::new(Point3::new(0.0, 1.5, -1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_intersects_ray() {
        let tri = Triangle::new(
            [
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-0.5, 0.0, 0.0),
                Point3::new(0.5, 0.0, 0.0),
            ],
            Rgb([255u8, 125u8, 12u8]),
        );
        let ray = Ray::new(Point3::new(0.0, 0.5, -1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), Some(Point3::new(0.0, 0.5, 0.0)));
    }

    #[test]
    fn triangle_can_be_multiplied_by_3d_matrix() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        #[rustfmt::skip]
        let mat = Matrix3::new(
            1.0, 2.0, 3.0,
            3.0, 1.5, -7.0,
            -3.14, 1.57, -3.0
        );
        let expected = Triangle::new(
            [
                mat * Point3::new(1.0, 0.0, 0.0),
                mat * Point3::new(0.0, 1.0, 0.0),
                mat * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(mat * tri, expected);
        assert_eq!(&mat * tri, expected);
        assert_eq!(mat * &tri, expected);
        assert_eq!(&mat * &tri, expected);
    }

    #[test]
    fn triangle_can_be_rotated() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        let rotation = Rotation3::new(Vector3::new(1.57, 0.0, -0.75));
        let expected = Triangle::new(
            [
                rotation * Point3::new(1.0, 0.0, 0.0),
                rotation * Point3::new(0.0, 1.0, 0.0),
                rotation * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(rotation * tri, expected);
        assert_eq!(&rotation * tri, expected);
        assert_eq!(rotation * &tri, expected);
        assert_eq!(&rotation * &tri, expected);
    }

    #[test]
    fn triangle_can_be_translated() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let expected = Triangle::new(
            [
                translation * Point3::new(1.0, 0.0, 0.0),
                translation * Point3::new(0.0, 1.0, 0.0),
                translation * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(translation * tri, expected);
        assert_eq!(&translation * tri, expected);
        assert_eq!(translation * &tri, expected);
        assert_eq!(&translation * &tri, expected);
    }

    #[test]
    fn triangle_can_be_rotated_and_translated() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        let isometry = Isometry3::new(Vector3::new(-1.0, 2.5, 0.0), Vector3::new(1.57, 0.0, -0.75));
        let expected = Triangle::new(
            [
                isometry * Point3::new(1.0, 0.0, 0.0),
                isometry * Point3::new(0.0, 1.0, 0.0),
                isometry * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(isometry * tri, expected);
        assert_eq!(&isometry * tri, expected);
        assert_eq!(isometry * &tri, expected);
        assert_eq!(&isometry * &tri, expected);
    }

    #[test]
    fn triangle_can_be_transformed_into_similar_ray() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let rotation = Unit::new_normalize(Quaternion::new(1.75, 0.0, 1.0, 2.0));
        let similarity = Similarity3::from_parts(translation, rotation, 2.0);
        let expected = Triangle::new(
            [
                similarity * Point3::new(1.0, 0.0, 0.0),
                similarity * Point3::new(0.0, 1.0, 0.0),
                similarity * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(similarity * tri, expected);
        assert_eq!(&similarity * tri, expected);
        assert_eq!(similarity * &tri, expected);
        assert_eq!(&similarity * &tri, expected);
    }

    #[test]
    fn triangle_can_be_transformed() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        #[rustfmt::skip]
        let transform = Transform3::from_matrix_unchecked(Matrix4::new(
            1.0, 2.0, 3.0, 0.0,
            3.0, 1.5, -7.0, 0.0,
            -3.14, 1.57, -3.0, 0.0,
            0.0, 1.57, 0.0, 1.0,
        ));
        let expected = Triangle::new(
            [
                transform * Point3::new(1.0, 0.0, 0.0),
                transform * Point3::new(0.0, 1.0, 0.0),
                transform * Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(transform * tri, expected);
        assert_eq!(&transform * tri, expected);
        assert_eq!(transform * &tri, expected);
        assert_eq!(&transform * &tri, expected);
    }

    #[test]
    fn triangle_can_be_multiplied_by_4d_matrix() {
        let tri = Triangle::new(
            [
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );
        #[rustfmt::skip]
        let matrix = Matrix4::new_scaling(2.0);
        let expected = Triangle::new(
            [
                matrix.transform_point(&Point3::new(1.0, 0.0, 0.0)),
                matrix.transform_point(&Point3::new(0.0, 1.0, 0.0)),
                matrix.transform_point(&Point3::new(-1.0, 0.0, 0.0)),
            ],
            Rgb([215u8, 225u8, 0u8]),
        );

        assert_eq!(matrix * tri, expected);
        assert_eq!(&matrix * tri, expected);
        assert_eq!(matrix * &tri, expected);
        assert_eq!(&matrix * &tri, expected);
    }
}
