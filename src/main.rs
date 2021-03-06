use image::{ImageBuffer, Rgb};
use nalgebra::{Isometry3, Point2, Point3, Rotation3, Similarity3, Translation3, Vector3};

use rustracer::primitives::Triangle;
use rustracer::{Colour, Material, Scalar, Scene, Viewport};

fn main() {
    let up_triangle = Triangle::new([
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
    ]);

    let world = Translation3::new(-1.0f32, -0.5, 0.0)
        * Rotation3::new(Vector3::new(0.0f32, std::f32::consts::PI * 0.1, 0.0));
    let mut scene = Scene::new(
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 0.0, green: 0.0, blue: 0.0,},
            #[rustfmt::skip]
            emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
        },
        5,
        5,
    );
    scene.add_triangle(
        world * up_triangle,
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
            #[rustfmt::skip]
            emission: Colour {red: 1.0, green: 1.0, blue: 0.0,},
        },
    );
    scene.add_triangle(
        world
            * Translation3::new(0.15f32, 0.15, 1.0)
            * (Similarity3::from_scaling(0.5f32) * up_triangle),
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
            ..Default::default()
        },
    );
    scene.add_triangle(
        world
            * Translation3::new(0.25f32, 0.25, 1.5)
            * (Similarity3::from_scaling(0.25f32) * up_triangle),
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 0.2, green: 1.0, blue: 0.0,},
            ..Default::default()
        },
    );
    scene.add_triangle(
        world
            * Translation3::new(0.0f32, -1.0, 6.0)
            * (Similarity3::from_scaling(2.5f32) * up_triangle),
        Material {
            #[rustfmt::skip]
            diffuse: Colour {red: 1.0, green: 1.0, blue: 1.0,},
            #[rustfmt::skip]
            emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
        },
    );

    let mut image_data = ImageBuffer::new(800, 600);

    let viewport = Viewport::new(
        image_data.width(),
        image_data.height(),
        std::f32::consts::PI / 2.0,
        1.0,
        1000.0,
        100,
    );

    let eye = Point3::new(0.0f32, 0.0, 5.0);
    let target = Point3::new(0.0f32, 0.0, 0.0);
    let camera = Isometry3::look_at_rh(&eye, &target, &Vector3::y()).inverse();

    for (x, y, pixel) in image_data.enumerate_pixels_mut() {
        let colour = viewport
            .cast_ray(Point2::new(x, y))
            .map(|ray| camera * ray)
            .map(|ray| scene.trace(&ray))
            .fold(Colour::default(), |acc, x| acc + x)
            / viewport.get_rays_count() as Scalar;
        let colour: Rgb<u8> = colour.into();
        *pixel = colour;
    }
    image_data.save("image.png").unwrap();
}
