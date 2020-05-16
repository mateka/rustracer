use nalgebra;

type Scalar = f32;
type Point3 = nalgebra::Point3<Scalar>;
type Vector3 = nalgebra::Vector3<Scalar>;
type Matrix3 = nalgebra::Matrix3<Scalar>;
type Rotation3 = nalgebra::Rotation3<Scalar>;
type Translation3 = nalgebra::Translation3<Scalar>;
type Isometry3 = nalgebra::Isometry3<Scalar>;
type Similarity3 = nalgebra::Similarity3<Scalar>;
type Transform3 = nalgebra::Transform3<Scalar>;
type Quaternion = nalgebra::Quaternion<Scalar>;

// type Vector4 = nalgebra::Vector4<Scalar>;
type Matrix4 = nalgebra::Matrix4<Scalar>;

mod ray;
pub use ray::Ray;
