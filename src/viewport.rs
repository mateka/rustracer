use nalgebra;

use crate::{Point3, Ray, Scalar};

pub type Perspective3 = nalgebra::Perspective3<Scalar>;
pub type ScreenPoint = nalgebra::Point2<u32>;

/// A viewport class
#[derive(Debug, Clone)]
pub struct Viewport {
    width: Scalar,
    height: Scalar,
    projection: Perspective3,
}

impl Viewport {
    pub fn new(width: u32, height: u32, fovy: Scalar, znear: Scalar, zfar: Scalar) -> Self {
        Self {
            width: width as Scalar,
            height: height as Scalar,
            projection: Perspective3::new(width as Scalar / height as Scalar, fovy, znear, zfar),
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

    pub fn normalize_point(&self, screen_point: ScreenPoint, z: Scalar) -> Point3 {
        Point3::new(
            (screen_point.x as Scalar / self.width as Scalar) - 0.5,
            0.5 - (screen_point.y as Scalar / self.height as Scalar),
            z,
        )
    }

    pub fn cast_ray(&self, screen_point: ScreenPoint) -> Ray {
        let near_point = self.normalize_point(screen_point, -1.0);
        let near_point = self.projection.unproject_point(&near_point);

        let far_point = self.normalize_point(screen_point, 1.0);
        let far_point = self.projection.unproject_point(&far_point);
        Ray::new(near_point, far_point - near_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_creation() {
        let vp = Viewport::new(640, 480, 2.0, 1.0, 100.0);
        assert_eq!(vp.get_width(), 640.0);
        assert_eq!(vp.get_height(), 480.0);
    }

    #[test]
    fn test_normalize_point() {
        let vp = Viewport::new(640, 480, 2.0, 1.0, 100.0);

        assert_eq!(
            vp.normalize_point(ScreenPoint::new(0, 0), 0.0),
            Point3::new(-0.5, 0.5, 0.0)
        );
        assert_eq!(
            vp.normalize_point(ScreenPoint::new(640, 0), 0.0),
            Point3::new(0.5, 0.5, 0.0)
        );
        assert_eq!(
            vp.normalize_point(ScreenPoint::new(0, 480), 0.0),
            Point3::new(-0.5, -0.5, 0.0)
        );
        assert_eq!(
            vp.normalize_point(ScreenPoint::new(640, 480), 0.0),
            Point3::new(0.5, -0.5, 0.0)
        );
        assert_eq!(
            vp.normalize_point(ScreenPoint::new(320, 240), 1.0),
            Point3::new(0.0, 0.0, 1.0)
        );
    }
}
