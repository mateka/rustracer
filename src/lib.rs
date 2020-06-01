pub type Scalar = f32;
pub type Point2 = nalgebra::Point2<Scalar>;
pub type Vector2 = nalgebra::Vector2<Scalar>;
pub type Point3 = nalgebra::Point3<Scalar>;
pub type Vector3 = nalgebra::Vector3<Scalar>;
pub type Matrix3 = nalgebra::Matrix3<Scalar>;
pub type Rotation3 = nalgebra::Rotation3<Scalar>;
pub type Translation3 = nalgebra::Translation3<Scalar>;
pub type Isometry3 = nalgebra::Isometry3<Scalar>;
pub type Similarity3 = nalgebra::Similarity3<Scalar>;
pub type Transform3 = nalgebra::Transform3<Scalar>;
pub type Quaternion = nalgebra::Quaternion<Scalar>;

// pub type Vector4 = nalgebra::Vector4<Scalar>;
pub type Matrix4 = nalgebra::Matrix4<Scalar>;

mod ray;
pub use ray::{Ray, RayTraceable};

mod viewport;
pub use viewport::{Perspective3, Viewport};

pub mod primitives;

mod material;
pub use material::{Colour, Material};

mod scene;
pub use scene::Scene;
