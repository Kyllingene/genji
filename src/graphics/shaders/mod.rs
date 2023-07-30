use glium::{Display, Program};

const SHAPE: (&str, &str) = (include_str!("shape.vert"), include_str!("shape.frag"));
const TEXTURE: (&str, &str) = (include_str!("texture.vert"), include_str!("texture.frag"));

pub struct Shaders {
    pub shape: Program,
    pub texture: Program,
}

impl Shaders {
    /// Initializes all the shaders.
    pub fn new(d: &Display) -> Self {
        Self {
            shape: Program::from_source(d, SHAPE.0, SHAPE.1, None).expect("error in shape shaders"),
            texture: Program::from_source(d, TEXTURE.0, TEXTURE.1, None)
                .expect("error in texture shaders"),
        }
    }
}
