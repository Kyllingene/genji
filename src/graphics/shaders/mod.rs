use glium::{Display, Program};

pub const SHAPE: (&str, &str) = (include_str!("shape.vert"), include_str!("shape.frag"));

pub struct Shaders {
    pub shape: Program,
}

impl Shaders {
    /// Initializes all the shaders.
    pub fn new(d: &Display) -> Self {
        Self {
            shape: Program::from_source(d, SHAPE.0, SHAPE.1, None).expect("error in shape shaders"),
        }
    }
}
