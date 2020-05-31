use image::{ImageBuffer, Rgb};
use nalgebra::{Isometry3, Point2, Point3, Rotation3, Similarity3, Translation3, Vector3};

use rustracer::primitives::Triangle;
use rustracer::{Colour, Material, Scene, Viewport};

fn main() {
    let up_triangle = Triangle::new([
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
    ]);

    let scale = Similarity3::from_scaling(0.5f32);
    let translate_red = Translation3::new(0.0f32, 0.4, 1.5);
    let translate_green = Translation3::new(0.0f32, 0.6, 2.5);
    let rotate = Rotation3::new(Vector3::new(0.0f32, 3.14 * 0.1, 0.0));
    let mut scene = Scene::new(
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 0.0, green: 0.0, blue: 0.0,},
            #[rustfmt::skip]
            emission: Colour {red: 0.75, green: 0.75, blue: 1.0,},
        },
        10,
    );
    scene.add_triangle(
        rotate * up_triangle,
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
            ..Default::default()
        },
    );
    scene.add_triangle(
        rotate * translate_red * (scale * up_triangle),
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
            ..Default::default()
        },
    );
    scene.add_triangle(
        rotate * translate_green * (scale * up_triangle),
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 0.2, green: 1.0, blue: 0.0,},
            ..Default::default()
        },
    );

    let mut image_data = ImageBuffer::new(400, 300);

    let viewport = Viewport::new(
        image_data.width(),
        image_data.height(),
        3.14 / 2.0,
        1.0,
        1000.0,
    );

    let eye = Point3::new(0.0f32, 0.0, 5.0);
    let target = Point3::new(0.0f32, 0.0, 0.0);
    let camera = Isometry3::look_at_rh(&eye, &target, &Vector3::y()).inverse();

    for (x, y, pixel) in image_data.enumerate_pixels_mut() {
        let ray = &camera * &viewport.cast_ray(Point2::new(x, y));
        let colour: Rgb<u8> = scene.trace(&ray).into();
        *pixel = colour;
    }
    image_data.save("image.png").unwrap();
}
