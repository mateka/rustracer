use auto_ops::*;
use nalgebra::Unit;

use crate::{
    ray::RayTraceable, Isometry3, Matrix3, Matrix4, Point2, Point3, Ray, Rotation3, Scalar,
    Similarity3, Transform3, Translation3, Vector3,
};

/// A triangle primitive
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    vertices: [Point3; 3],
    normal: Unit<Vector3>,
}

impl Triangle {
    pub fn new(vertices: [Point3; 3]) -> Self {
        Self {
            vertices: vertices,
            normal: Triangle::calculate_normal(vertices),
        }
    }

    pub fn get_v(&self, index: usize) -> &Point3 {
        &self.vertices[index]
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

    fn doubled_area_of(vertices: [Point3; 3]) -> Scalar {
        let v0v1 = vertices[1] - vertices[0];
        let v0v2 = vertices[2] - vertices[0];
        v0v1.cross(&v0v2).norm()
    }
}

impl RayTraceable for Triangle {
    fn get_normal(&self) -> Unit<Vector3> {
        self.normal
    }

    fn intersects(&self, ray: &Ray) -> Option<Point3> {
        let v0v1 = self.get_v(1) - self.get_v(0);
        let v0v2 = self.get_v(2) - self.get_v(0);
        let pvec = ray.direction.cross(&v0v2);
        let determinant = v0v1.dot(&pvec);
        if determinant.abs() <= Scalar::EPSILON {
            return None;
        }
        let inv_det = 1.0 / determinant;
        let tvec = ray.origin - self.get_v(0);
        let u = tvec.dot(&pvec) * inv_det;
        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;
        if u < 0.0 || u > 1.0 || v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if t < 0.0 {
            return None;
        }
        Some(ray.origin + t * ray.direction.into_inner())
    }

    fn local_2d_coordinates(&self, point: &Point3) -> Point2 {
        // See https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates
        let self_area = Triangle::doubled_area_of(self.vertices);
        let cap_area = Triangle::doubled_area_of([*self.get_v(0), *self.get_v(2), *point]);
        let abp_area = Triangle::doubled_area_of([*self.get_v(0), *self.get_v(1), *point]);
        Point2::new(cap_area / self_area, abp_area / self_area)
    }
}

impl_op_ex!(*|a: &Matrix3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(*|a: &Rotation3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(*|a: &Translation3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(*|a: &Isometry3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(
    *|a: &nalgebra::Isometry<Scalar, nalgebra::U3, Rotation3>, b: &Triangle| -> Triangle {
        Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
    }
);

impl_op_ex!(*|a: &Similarity3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(*|a: &Transform3, b: &Triangle| -> Triangle {
    Triangle::new([a * b.vertices[0], a * b.vertices[1], a * b.vertices[2]])
});

impl_op_ex!(*|a: &Matrix4, b: &Triangle| -> Triangle {
    Triangle::new([
        a.transform_point(&b.vertices[0]),
        a.transform_point(&b.vertices[1]),
        a.transform_point(&b.vertices[2]),
    ])
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quaternion;

    #[test]
    fn triangle_creation_normal_is_computed() {
        let tri = Triangle::new([
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
        ]);

        assert_eq!(tri.get_v(0), &Point3::new(0.0, 1.0, 0.0));
        assert_eq!(tri.get_v(1), &Point3::new(-0.5, 0.0, 0.0));
        assert_eq!(tri.get_v(2), &Point3::new(0.5, 0.0, 0.0));
        assert_eq!(
            tri.get_normal(),
            Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0))
        );
    }

    #[test]
    fn triangle_does_not_intersect_parallel_ray() {
        let tri = Triangle::new([
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
        ]);
        let ray = Ray::new(Point3::new(-1.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_does_not_intersect_ray_starting_behind_triangle() {
        let tri = Triangle::new([
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
        ]);
        let ray = Ray::new(Point3::new(0.0, 0.5, 1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_does_not_intersect_ray_hitting_triangle_plane_but_not_the_triangle() {
        let tri = Triangle::new([
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
        ]);
        let ray = Ray::new(Point3::new(0.0, 1.5, -1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), None);
    }

    #[test]
    fn triangle_intersects_ray() {
        let tri = Triangle::new([
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
            Point3::new(0.5, 0.0, 0.0),
        ]);
        let ray = Ray::new(Point3::new(0.0, 0.5, -1.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(tri.intersects(&ray), Some(Point3::new(0.0, 0.5, 0.0)));
    }

    #[test]
    fn triangle_can_be_multiplied_by_3d_matrix() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        #[rustfmt::skip]
        let mat = Matrix3::new(
            1.0, 2.0, 3.0,
            3.0, 1.5, -7.0,
            -3.14, 1.57, -3.0
        );
        let expected = Triangle::new([
            mat * Point3::new(1.0, 0.0, 0.0),
            mat * Point3::new(0.0, 1.0, 0.0),
            mat * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(mat * tri, expected);
        assert_eq!(&mat * tri, expected);
        assert_eq!(mat * &tri, expected);
        assert_eq!(&mat * &tri, expected);
    }

    #[test]
    fn triangle_can_be_rotated() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        let rotation = Rotation3::new(Vector3::new(1.57, 0.0, -0.75));
        let expected = Triangle::new([
            rotation * Point3::new(1.0, 0.0, 0.0),
            rotation * Point3::new(0.0, 1.0, 0.0),
            rotation * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(rotation * tri, expected);
        assert_eq!(&rotation * tri, expected);
        assert_eq!(rotation * &tri, expected);
        assert_eq!(&rotation * &tri, expected);
    }

    #[test]
    fn triangle_can_be_translated() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let expected = Triangle::new([
            translation * Point3::new(1.0, 0.0, 0.0),
            translation * Point3::new(0.0, 1.0, 0.0),
            translation * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(translation * tri, expected);
        assert_eq!(&translation * tri, expected);
        assert_eq!(translation * &tri, expected);
        assert_eq!(&translation * &tri, expected);
    }

    #[test]
    fn triangle_can_be_rotated_and_translated_by_two_transforms() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        let rotation = Rotation3::new(Vector3::new(1.57, 0.0, -0.75));
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let expected = Triangle::new([
            rotation * translation * Point3::new(1.0, 0.0, 0.0),
            rotation * translation * Point3::new(0.0, 1.0, 0.0),
            rotation * translation * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(rotation * translation * tri, expected);
        assert_eq!(rotation * translation * &tri, expected);
    }

    #[test]
    fn triangle_can_be_rotated_and_translated() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        let isometry = Isometry3::new(Vector3::new(-1.0, 2.5, 0.0), Vector3::new(1.57, 0.0, -0.75));
        let expected = Triangle::new([
            isometry * Point3::new(1.0, 0.0, 0.0),
            isometry * Point3::new(0.0, 1.0, 0.0),
            isometry * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(isometry * tri, expected);
        assert_eq!(&isometry * tri, expected);
        assert_eq!(isometry * &tri, expected);
        assert_eq!(&isometry * &tri, expected);
    }

    #[test]
    fn triangle_can_be_transformed_into_similar_ray() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        let translation = Translation3::new(-1.0, 2.5, 0.0);
        let rotation = Unit::new_normalize(Quaternion::new(1.75, 0.0, 1.0, 2.0));
        let similarity = Similarity3::from_parts(translation, rotation, 2.0);
        let expected = Triangle::new([
            similarity * Point3::new(1.0, 0.0, 0.0),
            similarity * Point3::new(0.0, 1.0, 0.0),
            similarity * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(similarity * tri, expected);
        assert_eq!(&similarity * tri, expected);
        assert_eq!(similarity * &tri, expected);
        assert_eq!(&similarity * &tri, expected);
    }

    #[test]
    fn triangle_can_be_transformed() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        #[rustfmt::skip]
        let transform = Transform3::from_matrix_unchecked(Matrix4::new(
            1.0, 2.0, 3.0, 0.0,
            3.0, 1.5, -7.0, 0.0,
            -3.14, 1.57, -3.0, 0.0,
            0.0, 1.57, 0.0, 1.0,
        ));
        let expected = Triangle::new([
            transform * Point3::new(1.0, 0.0, 0.0),
            transform * Point3::new(0.0, 1.0, 0.0),
            transform * Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(transform * tri, expected);
        assert_eq!(&transform * tri, expected);
        assert_eq!(transform * &tri, expected);
        assert_eq!(&transform * &tri, expected);
    }

    #[test]
    fn triangle_can_be_multiplied_by_4d_matrix() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);
        #[rustfmt::skip]
        let matrix = Matrix4::new_scaling(2.0);
        let expected = Triangle::new([
            matrix.transform_point(&Point3::new(1.0, 0.0, 0.0)),
            matrix.transform_point(&Point3::new(0.0, 1.0, 0.0)),
            matrix.transform_point(&Point3::new(-1.0, 0.0, 0.0)),
        ]);

        assert_eq!(matrix * tri, expected);
        assert_eq!(&matrix * tri, expected);
        assert_eq!(matrix * &tri, expected);
        assert_eq!(&matrix * &tri, expected);
    }

    #[test]
    fn triangle_vertices_2d_coordinates() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(
            Point2::new(0.0, 0.0),
            tri.local_2d_coordinates(&Point3::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Point2::new(1.0, 0.0),
            tri.local_2d_coordinates(&Point3::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Point2::new(0.0, 1.0),
            tri.local_2d_coordinates(&Point3::new(-1.0, 0.0, 0.0))
        );
    }

    #[test]
    fn triangle_inside_points_2d_coordinates() {
        let tri = Triangle::new([
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ]);

        assert_eq!(
            Point2::new(0.5, 0.25),
            tri.local_2d_coordinates(&Point3::new(0.0, 0.5, 0.0))
        );
        assert_eq!(
            Point2::new(0.5, 0.5),
            tri.local_2d_coordinates(&Point3::new(-0.5, 0.5, 0.0))
        );
    }
}
