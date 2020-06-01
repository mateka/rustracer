use crate::Colour;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Material {
    pub emission: Colour,
    pub diffuse: Colour,
}
