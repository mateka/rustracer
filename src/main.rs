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

    let scale02 = Similarity3::from_scaling(0.2f32);
    let translate_x21 = Translation3::new(2.1f32, 0.0, 0.0);
    let translate_x_21 = Translation3::new(-2.1f32, 0.0, 0.0);
    let rotate = Rotation3::new(Vector3::new(0.0f32, 0.0, 3.14));

    let mut scene = Scene::new(Colour {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    });
    scene.add_triangle(
        up_triangle,
        Material {
            colour: Colour {
                red: 1.0,
                green: 1.0,
                blue: 0.0,
            },
        },
    );
    scene.add_triangle(
        translate_x21 * scale02 * up_triangle,
        Material {
            colour: Colour {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
        },
    );
    scene.add_triangle(
        translate_x_21.to_homogeneous()
            * scale02.to_homogeneous()
            * rotate.to_homogeneous()
            * up_triangle,
        Material {
            colour: Colour {
                red: 0.0,
                green: 1.0,
                blue: 0.0,
            },
        },
    );
    scene.add_triangle(
        Rotation3::new(Vector3::new(0.0, 3.14 / 4.0, 0.0)) * up_triangle,
        Material {
            colour: Colour {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
            },
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
