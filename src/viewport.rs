use crate::{Point2, Point3, Ray, Scalar, Vector2};
use noise::{NoiseFn, OpenSimplex};

pub type Perspective3 = nalgebra::Perspective3<Scalar>;
pub type ScreenPoint = nalgebra::Point2<u32>;

/// A viewport class
#[derive(Debug, Clone)]
pub struct Viewport {
    width: Scalar,
    height: Scalar,
    projection: Perspective3,
    point_offsets: Vec<Vector2>,
}

impl Viewport {
    pub fn new(
        width: u32,
        height: u32,
        fovy: Scalar,
        znear: Scalar,
        zfar: Scalar,
        point_rays_count: usize,
    ) -> Self {
        let noise = OpenSimplex::new();
        let coords = (0..point_rays_count)
            .map(|_| {
                let pt = [0.0, 0.0];
                Vector2::new(noise.get(pt) as f32, noise.get(pt) as f32)
            })
            .map(|v| v * 0.5);
        Self {
            width: width as Scalar,
            height: height as Scalar,
            projection: Perspective3::new(width as Scalar / height as Scalar, fovy, znear, zfar),
            point_offsets: coords.collect(),
        }
    }

    pub fn get_width(&self) -> Scalar {
        self.width
    }

    pub fn get_height(&self) -> Scalar {
        self.height
    }

    pub fn get_projection(&self) -> &Perspective3 {
        &self.projection
    }

    pub fn get_rays_count(&self) -> usize {
        self.point_offsets.len()
    }

    pub fn normalize_point(&self, screen_point: Point2) -> Point2 {
        Point2::new(
            screen_point.x / self.width - 0.5,
            0.5 - screen_point.y / self.height,
        )
    }

    pub fn cast_ray<'a>(&'a self, screen_point: ScreenPoint) -> impl Iterator<Item = Ray> + 'a {
        self.point_offsets
            .iter()
            .map(move |off| {
                self.normalize_point(
                    Point2::new(screen_point.x as Scalar, screen_point.y as Scalar) + off,
                )
            })
            .map(|pt| {
                (
                    Point3::new(pt.coords[0], pt.coords[1], -1.0),
                    Point3::new(pt.coords[0], pt.coords[1], 1.0),
                )
            })
            .map(move |(near, far)| {
                (
                    self.projection.unproject_point(&near),
                    self.projection.unproject_point(&far),
                )
            })
            .map(|(near, far)| Ray::new(near, far - near))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_creation() {
        let vp = Viewport::new(640, 480, 2.0, 1.0, 100.0, 2);
        assert!(vp.get_width() - 640.0 <= std::f32::EPSILON);
        assert!(vp.get_height() - 480.0 <= std::f32::EPSILON);
        assert_eq!(vp.get_rays_count(), 2);
    }

    #[test]
    fn ray_casting() {
        let vp = Viewport::new(640, 480, 2.0, 1.0, 100.0, 25);
        let rays: Vec<Ray> = vp.cast_ray(ScreenPoint::new(320, 240)).collect();
        assert_eq!(rays.len(), 25);
        for ray in rays {
            assert!((ray.origin - Point3::new(0.0, 0.0, -1.0)).norm() <= 0.5_f32.sqrt());
        }
    }

    #[test]
    fn test_normalize_point() {
        let vp = Viewport::new(640, 480, 2.0, 1.0, 100.0, 1);

        assert_eq!(
            vp.normalize_point(Point2::new(0.0, 0.0)),
            Point2::new(-0.5, 0.5)
        );
        assert_eq!(
            vp.normalize_point(Point2::new(640.0, 0.0)),
            Point2::new(0.5, 0.5)
        );
        assert_eq!(
            vp.normalize_point(Point2::new(0.0, 480.0)),
            Point2::new(-0.5, -0.5)
        );
        assert_eq!(
            vp.normalize_point(Point2::new(640.0, 480.0)),
            Point2::new(0.5, -0.5)
        );
        assert_eq!(
            vp.normalize_point(Point2::new(320.0, 240.0)),
            Point2::new(0.0, 0.0)
        );
    }
}
