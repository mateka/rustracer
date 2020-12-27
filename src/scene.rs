use crate::{
    primitives::Triangle, Colour, Material, Point3, Ray, RayTraceable, Rotation3, Scalar, Vector3,
};
use nalgebra::{Reflection, Unit};
use rand::prelude::*;

/// Helper struct describing hit result
#[derive(Debug, PartialEq, Copy, Clone)]
struct HitResult {
    pub point: Point3,
    pub index: usize,
}

/// Helper struct describing trace result
#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct TraceResult {
    pub diffuse: Colour,
    pub emission: Colour,
}

/// Scene helper to organize and ray trace primitives of one type
#[derive(Debug, PartialEq, Clone)]
struct PrimitivesWithMaterials<P: RayTraceable> {
    primitives: Vec<P>,
    materials: Vec<Material>,
}

/// Ray traceable scene
#[derive(Debug, PartialEq, Clone)]
pub struct Scene {
    default_material: Material,
    recursion_depth: usize,
    beam_rays_count: usize,
    triangles: PrimitivesWithMaterials<Triangle>,
}

impl Scene {
    /// Creates new scene
    pub fn new(default_material: Material, recursion_depth: usize, beam_rays_count: usize) -> Self {
        Self {
            default_material,
            recursion_depth,
            beam_rays_count,
            triangles: PrimitivesWithMaterials::new(),
        }
    }

    /// Adds triangle to the scene
    pub fn add_triangle(&mut self, triangle: Triangle, material: Material) {
        self.triangles.add(triangle, material)
    }

    /// Traces ray emission
    pub fn trace(&self, ray: &Ray) -> Colour {
        self.trace_until(ray, 0).diffuse
    }

    fn trace_until(&self, ray: &Ray, step: usize) -> TraceResult {
        let hit = self.closest_hit(ray);
        if hit == None {
            return TraceResult::from(self.default_material);
        }
        let hit = hit.unwrap();
        let mut trace_result = TraceResult {
            ..Default::default()
        };
        let material = self.triangles.get_material(hit.index);

        if step < self.recursion_depth {
            let reflected_ray = self.get_reflected_ray(&ray, &hit);
            let primitive_size = self.triangles.get_primitive(hit.index).get_size();
            for beam_ray in self.get_beam(reflected_ray, primitive_size) {
                let tr = self.trace_until(&beam_ray, step + 1);
                trace_result.add_light(&tr);
            }
            trace_result.emission /= self.beam_rays_count as Scalar;
        } else {
            trace_result = TraceResult::from(self.default_material);
        }
        trace_result.apply_to(material)
    }

    fn closest_hit(&self, ray: &Ray) -> Option<HitResult> {
        self.triangles.closest_hit(ray)
    }

    fn get_reflected_ray(&self, ray: &Ray, hit: &HitResult) -> Ray {
        let mut vector = ray.direction.into_inner().clone_owned();
        let reflection = Reflection::new_containing_point(
            self.triangles.get_primitive(hit.index).get_normal(),
            &hit.point,
        );
        reflection.reflect(&mut vector);
        let reflected_direction = Unit::new_normalize(vector);
        Ray {
            // Move ray origin away from target in order to avoid infinite self reflections
            origin: hit.point + 2.0 * Scalar::EPSILON * reflected_direction.into_inner(),
            direction: reflected_direction,
        }
    }

    fn get_beam(&self, ray: Ray, primitive_size: Scalar) -> impl Iterator<Item = Ray> {
        let rotation =
            Rotation3::rotation_between(&Vector3::new(0.0, 0.0, 1.0), &ray.direction.into_inner())
                .unwrap_or_else(Rotation3::identity);
        let mut randomness = thread_rng();
        (0..self.beam_rays_count)
            .map(move |_| {
                let r = randomness.gen_range(0.0..(0.005 * primitive_size));
                let alpha = randomness.gen_range(0.0..(2.0 * std::f32::consts::PI));
                Vector3::new(r * alpha.cos(), r * alpha.sin(), 0.0)
            })
            .map(move |circle_vec| rotation * circle_vec)
            .map(move |v| Ray::new(ray.origin, ray.direction.into_inner() + v))
    }
}

impl<P: RayTraceable> PrimitivesWithMaterials<P> {
    /// Creates new Scene helper
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            materials: Vec::new(),
        }
    }
    /// Adds primitive with material and keeps indices synchronized
    pub fn add(&mut self, primitive: P, material: Material) {
        self.primitives.push(primitive);
        self.materials.push(material);
    }

    /// Finds primitive closest to ray's origin
    pub fn closest_hit(&self, ray: &Ray) -> Option<HitResult> {
        let hit = self
            .primitives
            .iter()
            .enumerate()
            .filter_map(|(i, t)| match t.intersects(&ray) {
                Some(p) => Some((i, p)),
                None => None,
            })
            .map(|(i, p)| (i, p, (p - ray.origin).norm()))
            .filter(|(_, _, d)| !d.is_nan())
            .min_by(|&(_, _, d1), &(_, _, d2)| d1.partial_cmp(&d2).unwrap())?;

        Some(HitResult {
            point: hit.1,
            index: hit.0,
        })
    }

    pub fn get_primitive(&self, index: usize) -> &P {
        &self.primitives[index]
    }

    pub fn get_material(&self, index: usize) -> &Material {
        &self.materials[index]
    }
}

impl TraceResult {
    pub fn add_light(&mut self, other: &Self) {
        self.emission += other.emission;
    }

    pub fn apply_to(&self, material: &Material) -> Self {
        Self {
            emission: material.emission + material.diffuse * self.emission,
            diffuse: material.emission + material.diffuse * self.emission,
        }
    }
}

impl From<Material> for TraceResult {
    fn from(material: Material) -> Self {
        Self {
            emission: material.emission,
            diffuse: material.diffuse,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rotation3, Translation3, Vector3};

    #[test]
    fn tracing_scene_with_bouncing_rays_and_solid_triangles() {
        let mut scene = Scene::new(
            Material {
                #[rustfmt::skip]
                diffuse: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                #[rustfmt::skip]
                emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
            },
            2,
            1,
        );
        let triangle = Triangle::new([
            Point3::new(1.0, -1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, -1.0, 0.0),
        ]);
        let rotation = Rotation3::new(Vector3::new(0.0f32, std::f32::consts::PI * 0.1, 0.0));

        // Yellow triangle partially hidden by red triangle. Both rotated,
        // to get red 'shadow' on yellow triangle.
        scene.add_triangle(
            rotation * triangle,
            Material {
                #[rustfmt::skip]
                diffuse: Colour {red: 0.75, green: 1.0, blue: 0.0,},
                ..Default::default()
            },
        );
        scene.add_triangle(
            rotation * Translation3::new(0.5f32, 0.0, 2.0) * triangle,
            Material {
                #[rustfmt::skip]
                diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
                ..Default::default()
            },
        );
        // Ray into abyss
        let ray = Ray::new(
            Point3::new(-0.666, 0.499, 4.0),
            Vector3::new(-0.511, 0.383, -0.768),
        );
        #[rustfmt::skip]
        assert_eq!(
            Colour{red: 0.0, green: 0.0, blue: 0.0}, scene.trace(&ray),
            "ray should not hit anything"
        );
        // Ray into yellow triangle
        let ray = Ray::new(
            Point3::new(0.04329_888, 0.07993_634, 4.0),
            Vector3::new(0.04312_106, 0.07960_805, -0.9958_931),
        );
        #[rustfmt::skip]
        assert_eq!(
            Colour{red: 0.75, green: 1.0, blue: 0.0}, scene.trace(&ray),
            "ray should hit yellow triangle"
        );
        // Ray into red triangle
        let ray = Ray::new(
            Point3::new(0.333, 0.166, 4.0),
            Vector3::new(0.312, 0.156, -0.93),
        );
        #[rustfmt::skip]
        assert_eq!(
            Colour{red: 1.0, green: 0.0, blue: 0.0}, scene.trace(&ray),
            "ray should hit red triangle"
        );
        // Ray into red triangle "shadow" over yellow triangle
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, -1.0));
        #[rustfmt::skip]
        assert_eq!(
            Colour{red: 0.75, green: 0.0, blue: 0.0}, scene.trace(&ray),
            "ray should hit yellow triangle on red triangle's shadow"
        );
    }

    mod no_recusrion {
        use super::*;

        #[test]
        fn tracing_empty_scene_yields_default_colour() {
            let scene = Scene::new(
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                },
                0,
                1,
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            #[rustfmt::skip]
            assert_eq!(Colour{red: 1.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
        }

        #[test]
        fn tracing_with_miss_yields_default_colour() {
            let mut scene = Scene::new(
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                },
                0,
                1,
            );
            scene.add_triangle(
                Triangle::new([
                    Point3::new(2.0, 2.0, 0.0),
                    Point3::new(1.5, 2.5, 0.0),
                    Point3::new(1.0, 2.0, 0.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            #[rustfmt::skip]
            assert_eq!(Colour{red: 1.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
        }

        #[test]
        fn tracing_with_hit_yields_hitted_primitive_colour() {
            let mut scene = Scene::new(
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                },
                0,
                1,
            );
            scene.add_triangle(
                Triangle::new([
                    Point3::new(1.0, -1.0, 0.0),
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(-1.0, -1.0, 0.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 0.0, green: 1.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            #[rustfmt::skip]
            assert_eq!(Colour{red: 0.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
        }

        #[test]
        fn tracing_with_hit_yields_closest_hitted_primitive_colour() {
            let mut scene = Scene::new(
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 1.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                },
                0,
                1,
            );
            scene.add_triangle(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.1),
                    Point3::new(0.0, 1.0, 1.1),
                    Point3::new(-1.0, -1.0, 1.1),
                ]),
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            scene.add_triangle(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 0.0, green: 1.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            #[rustfmt::skip]
            assert_eq!(Colour{red: 0.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
        }
    }

    mod primitives_with_materials_tests {
        use super::*;

        #[test]
        fn test_getters() {
            let mut primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.1),
                    Point3::new(0.0, 1.0, 1.1),
                    Point3::new(-1.0, -1.0, 1.1),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                    ..Default::default()
                },
            );
            assert_eq!(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                *primitives.get_primitive(1)
            );
            assert_eq!(
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0},
                    ..Default::default()
                },
                *primitives.get_material(1)
            );
        }

        #[test]
        fn closest_hit_is_empty_for_empty_list() {
            let primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            assert_eq!(None, primitives.closest_hit(&ray));
        }

        #[test]
        fn closest_hit_is_empty_for_miss() {
            let mut primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            primitives.add(
                Triangle::new([
                    Point3::new(2.0, 2.0, 0.0),
                    Point3::new(1.5, 2.5, 0.0),
                    Point3::new(1.0, 2.0, 0.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            assert_eq!(None, primitives.closest_hit(&ray));
        }

        #[test]
        fn closest_hit_is_nonempty_for_hit() {
            let mut primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 0.0),
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(-1.0, -1.0, 0.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            assert_eq!(
                Some(HitResult {
                    index: 0,
                    point: Point3::new(0.0, 0.0, 0.0)
                }),
                primitives.closest_hit(&ray)
            );
        }

        #[test]
        fn closest_hit_chooses_closest_primitive_on_hit() {
            let mut primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.1),
                    Point3::new(0.0, 1.0, 1.1),
                    Point3::new(-1.0, -1.0, 1.1),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                Material {
                    #[rustfmt::skip]
                    emission: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    ..Default::default()
                },
            );
            let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
            assert_eq!(
                Some(HitResult {
                    index: 1,
                    point: Point3::new(0.0, 0.0, 1.0)
                }),
                primitives.closest_hit(&ray)
            );
        }
    }

    mod trace_result_tests {
        use super::*;

        #[test]
        fn trace_result_can_be_created_from_material() {
            let material = Material {
                #[rustfmt::skip]
                diffuse: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                #[rustfmt::skip]
                emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
            };
            let trace_result = TraceResult::from(material);
            assert_eq!(
                trace_result,
                TraceResult {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 0.0, green: 0.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 1.0,},
                }
            );
        }

        #[test]
        fn trace_results_can_add_lights() {
            let mut tr1 = TraceResult {
                #[rustfmt::skip]
                diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
                #[rustfmt::skip]
                emission: Colour {red: 1.0, green: 0.0, blue: 0.25,},
            };
            let tr2 = TraceResult {
                #[rustfmt::skip]
                diffuse: Colour {red: 0.0, green: 1.0, blue: 0.0,},
                #[rustfmt::skip]
                emission: Colour {red: 0.0, green: 1.0, blue: 0.35,},
            };
            tr1.add_light(&tr2);
            assert_eq!(
                tr1,
                TraceResult {
                    #[rustfmt::skip]
                    diffuse: Colour {red: 1.0, green: 0.0, blue: 0.0,},
                    #[rustfmt::skip]
                    emission: Colour {red: 1.0, green: 1.0, blue: 0.6,},
                }
            );
        }
    }
}
