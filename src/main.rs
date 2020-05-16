use image::{ImageBuffer, Rgb};
use nalgebra::{
    geometry::Perspective3, Isometry3, Point2, Point3, Rotation3, Similarity3, Translation3, Unit,
    Vector3,
};

use rustracer::primitives::Triangle;
use rustracer::{Ray, RayTraceable};

struct Viewport {
    width: f32,
    height: f32,
    projection: Perspective3<f32>,
}

impl Viewport {
    fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Viewport {
        Viewport {
            width: width as f32,
            height: height as f32,
            projection: Perspective3::new(width as f32 / height as f32, fovy, znear, zfar),
        }
    }

    fn cast_ray(&self, screen_point: Point2<u32>) -> Ray {
        let near_point = Point3::new(
            -(screen_point.x as f32 / self.width as f32) + 0.5,
            -(screen_point.y as f32 / self.height as f32) + 0.5,
            -1.0,
        );
        let near_point = self.projection.unproject_point(&near_point);
        let far_point = Point3::new(
            -(screen_point.x as f32 / self.width as f32) + 0.5,
            -(screen_point.y as f32 / self.height as f32) + 0.5,
            1.0,
        );
        let far_point = self.projection.unproject_point(&far_point);
        Ray {
            origin: near_point,
            direction: Unit::new_normalize(far_point - near_point),
        }
    }
}

fn main() {
    let up_triangle = Triangle::new(
        [
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
        ],
        Rgb([215u8, 225u8, 0u8]),
    );

    let scale02 = Similarity3::from_scaling(0.2f32);
    let translate_x21 = Translation3::new(2.1f32, 0.0, 0.0);
    let translate_x_21 = Translation3::new(-2.1f32, 0.0, 0.0);
    let rotate = Rotation3::new(Vector3::new(0.0f32, 0.0, 3.14));

    let scene = vec![
        up_triangle,
        Triangle::new_triangle_colour(
            &(translate_x21 * scale02 * up_triangle),
            Rgb([215u8, 0u8, 0u8]),
        ),
        Triangle::new_triangle_colour(
            &(translate_x_21.to_homogeneous()
                * scale02.to_homogeneous()
                * rotate.to_homogeneous()
                * up_triangle),
            Rgb([0u8, 225u8, 0u8]),
        ),
        Triangle::new_triangle_colour(
            &(Rotation3::new(Vector3::new(0.0, 3.14 / 4.0, 0.0)) * up_triangle),
            Rgb([0u8, 0u8, 215u8]),
        ),
    ];

    let mut image_data = ImageBuffer::new(400, 300);

    let viewport = Viewport::new(
        image_data.width(),
        image_data.height(),
        3.14 / 2.0,
        1.0,
        1000.0,
    );

    let eye = Point3::new(0.0f32, 0.0, -5.0);
    let target = Point3::new(0.0f32, 0.0, 0.0);
    let camera = Isometry3::look_at_rh(&eye, &target, &Vector3::y()).inverse();

    for (x, y, pixel) in image_data.enumerate_pixels_mut() {
        let ray = &camera * &viewport.cast_ray(Point2::new(x, y));
        let mut distance = 100000.0f32;
        let mut last_colour = Rgb([0u8, 0u8, 0u8]);
        for triangle in &scene {
            match triangle.intersects(&ray) {
                Some(p) => {
                    let new_distance = (p - eye).norm();
                    if new_distance < distance {
                        last_colour = triangle.colour;
                        distance = new_distance;
                    }
                }
                None => (),
            }
        }
        *pixel = last_colour;
    }
    image_data.save("image.png").unwrap();
}
