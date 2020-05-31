use crate::Colour;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Material {
    pub emission: Colour,
    pub diffuse: Colour,
}

impl Material {
    pub fn combine(&mut self, other: &Material) {
        self.diffuse *= other.emission;
        self.emission += self.diffuse;
    }
}
