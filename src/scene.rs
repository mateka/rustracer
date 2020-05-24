use crate::{primitives::Triangle, Colour, Material, Point3, Ray, RayTraceable};

/// Helper struct describing hit result
#[derive(Debug, PartialEq, Copy, Clone)]
struct HitResult {
    pub point: Point3,
    pub index: usize,
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
    default_colour: Colour,
    triangles: PrimitivesWithMaterials<Triangle>,
}

impl Scene {
    /// Creates new scene
    pub fn new(default_colour: Colour) -> Self {
        Self {
            default_colour: default_colour,
            triangles: PrimitivesWithMaterials::new(),
        }
    }

    /// Adds triangle to the scene
    pub fn add_triangle(&mut self, triangle: Triangle, material: Material) {
        self.triangles.add(triangle, material)
    }

    /// Traces ray colour
    pub fn trace(&self, ray: &Ray) -> Colour {
        match self.triangles.closest_hit(ray) {
            Some(hr) => self.triangles.get_material(hr.index).colour,
            None => self.default_colour,
        }
    }
}

impl<P: RayTraceable> PrimitivesWithMaterials<P> {
    /// Creates new Scene helper
    pub fn new() -> Self {
        return Self {
            primitives: Vec::new(),
            materials: Vec::new(),
        };
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
            .min_by(|&(_, _, d1), &(_, _, d2)| d1.partial_cmp(&d2).unwrap())?;

        Some(HitResult {
            point: hit.1,
            index: hit.0,
        })
    }

    pub fn get_material(&self, index: usize) -> &Material {
        &self.materials[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector3;
    #[test]
    fn tracing_empty_scene_yields_default_colour() {
        let scene = Scene::new(Colour {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
        });
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        #[rustfmt::skip]
        assert_eq!(Colour{red: 1.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
    }

    #[test]
    fn tracing_with_miss_yields_default_colour() {
        let mut scene = Scene::new(Colour {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
        });
        scene.add_triangle(
            Triangle::new([
                Point3::new(2.0, 2.0, 0.0),
                Point3::new(1.5, 2.5, 0.0),
                Point3::new(1.0, 2.0, 0.0),
            ]),
            Material {
                colour: Colour {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                },
            },
        );
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        #[rustfmt::skip]
        assert_eq!(Colour{red: 1.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
    }

    #[test]
    fn tracing_with_hit_yields_hitted_primitive_colour() {
        let mut scene = Scene::new(Colour {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
        });
        scene.add_triangle(
            Triangle::new([
                Point3::new(1.0, -1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(-1.0, -1.0, 0.0),
            ]),
            Material {
                colour: Colour {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            },
        );
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
    }

    #[test]
    fn tracing_with_hit_yields_closest_hitted_primitive_colour() {
        let mut scene = Scene::new(Colour {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
        });
        scene.add_triangle(
            Triangle::new([
                Point3::new(1.0, -1.0, 1.1),
                Point3::new(0.0, 1.0, 1.1),
                Point3::new(-1.0, -1.0, 1.1),
            ]),
            Material {
                colour: Colour {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
            },
        );
        scene.add_triangle(
            Triangle::new([
                Point3::new(1.0, -1.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
                Point3::new(-1.0, -1.0, 1.0),
            ]),
            Material {
                colour: Colour {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            },
        );
        let ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.0, green: 1.0, blue: 0.0}, scene.trace(&ray));
    }

    mod primitives_with_materials_tests {
        use super::*;

        #[test]
        fn test_material_getter() {
            let mut primitives: PrimitivesWithMaterials<Triangle> = PrimitivesWithMaterials::new();
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.1),
                    Point3::new(0.0, 1.0, 1.1),
                    Point3::new(-1.0, -1.0, 1.1),
                ]),
                Material {
                    colour: Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
                },
            );
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                Material {
                    colour: Colour {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                    },
                },
            );
            assert_eq!(
                Material {
                    colour: Colour {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0
                    },
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
                    colour: Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
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
                    colour: Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
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
                    colour: Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
                },
            );
            primitives.add(
                Triangle::new([
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ]),
                Material {
                    colour: Colour {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
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
}
