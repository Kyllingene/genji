use std::{f32::consts::PI, fs::File, io::BufReader, path::Path};

use crate::helpers::gj2gl;

pub(crate) mod shaders;
// use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use shaders::Shaders;

use glium::{implement_vertex, uniform, Display, Frame, PolygonMode, Surface, VertexBuffer, Blend};

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    normal: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, color, tex_coords);

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// A color in RGBA, using the 0-255 range.
impl Color {
    /// Creates an opaque white color.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(mut self, r: u8) -> Self {
        self.r = r;
        self
    }

    pub fn g(mut self, g: u8) -> Self {
        self.g = g;
        self
    }

    pub fn b(mut self, b: u8) -> Self {
        self.b = b;
        self
    }

    pub fn a(mut self, a: u8) -> Self {
        self.a = a;
        self
    }

    /// Adjusts a color from 0.0-1.0 to 0-255.
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    /// Adjusts a color from 0-255 to 0.0-1.0.
    pub fn to_f32(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

/// The data underlying every sprite.
#[derive(Debug, Clone, Copy)]
pub struct SpriteData {
    /// The horizontal position of the sprite.
    /// Defaults to `0`.
    pub x: i32,
    /// The vertical position of the sprite.
    /// Defaults to `0`.
    pub y: i32,
    /// The z-level of the sprite. `0` hides it.
    /// Defaults to `1`.
    pub depth: u32,
    /// The rotation of the sprite, in degrees.
    /// Defaults to `0.0`.
    pub angle: f32,
    /// Whether or not to fill the sprite (only for shapes).
    /// Defaults to `true`.
    pub fill: bool,
    /// The weight of the lines if `!fill` (only for shapes).
    /// Defaults to `4`.
    pub stroke_weight: u32,
    /// The color of the sprite (for sprites, offsets the color).
    /// Defaults to opaque white.
    pub color: Color,
}

impl SpriteData {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn stroke_weight(mut self, stroke_weight: u32) -> Self {
        self.stroke_weight = stroke_weight;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn xy(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            depth: 1,
            fill: true,
            angle: 0.0,
            stroke_weight: 4,
            color: Color::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sprite {
    Rect {
        w: i32,
        h: i32,
        ex: SpriteData,
    },
    Circle {
        r: i32,
        ex: SpriteData,
    },
    Triangle {
        w: i32,
        h: i32,
        ex: SpriteData,
    },
    Text {
        s: String,
        font_size: i32,
        ex: SpriteData,
    },
    Texture {
        path: String,
        data: Vec<u8>,
        dimensions: (u32, u32),
        w: i32,
        h: i32,
        ex: SpriteData,
    },
}

impl Sprite {
    pub fn rect(w: i32, h: i32, ex: SpriteData) -> Self {
        Self::Rect { w, h, ex }
    }

    pub fn circle(r: i32, ex: SpriteData) -> Self {
        Self::Circle { r, ex }
    }

    pub fn triangle(w: i32, h: i32, ex: SpriteData) -> Self {
        Self::Triangle { w, h, ex }
    }

    pub fn text(s: String, font_size: i32, ex: SpriteData) -> Self {
        Self::Text { s, font_size, ex }
    }

    pub fn texture<S: ToString>(path: S, w: i32, h: i32, ex: SpriteData) -> Option<Self> {
        let path = path.to_string();
        let image = image::load(
            BufReader::new(File::open(&path).ok()?),
            image::ImageFormat::from_extension(
                Path::new(&path)
                    .extension()
                    .map(|e| e.to_str().unwrap_or(""))?,
            )?,
        )
        .ok()?
        .to_rgba8();

        let dimensions = image.dimensions();

        Some(Self::Texture {
            path,
            data: image.into_raw(),
            dimensions,
            w,
            h,
            ex,
        })
    }

    pub fn draw(&self, target: &mut Frame, d: &Display, shaders: &Shaders) {
        let ex = self.sprite_data();

        let mut params = glium::DrawParameters {
            // depth: glium::Depth {
            //     test: glium::draw_parameters::DepthTest::IfLess,
            //     write: true,
            //     ..Default::default()
            // },
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let color = ex.color.to_f32();

        let indices = if ex.fill {
            glium::index::PrimitiveType::TriangleStrip
        } else {
            params.polygon_mode = PolygonMode::Line;
            params.line_width = Some(gj2gl::coord(ex.stroke_weight as i32 + 500));
            glium::index::PrimitiveType::LineStrip
        };

        let (s_width, s_height) = target.get_dimensions();
        let ratio = s_height as f32 / s_width as f32;
        let a = -ex.angle * (PI / 180.0);
        let mat = [
            [a.cos() * ratio, a.sin(), 0.0, 0.0],
            [-a.sin(), a.cos(), 0.0, 0.0],
            [0.0, 0.0, (ex.depth as f32) / 256.0, 0.0],
            [gj2gl::coord(ex.x), gj2gl::coord(ex.y), 0.0, 1.0],
        ];

        let uniforms = uniform! {
            matrix: mat,
        };

        match self {
            Sprite::Rect { w, h, .. } => {
                let w = gj2gl::coord(*w) / 2.0;
                let h = gj2gl::coord(*h) / 2.0;

                let vb = if ex.fill {
                    let vertices = [
                        Vertex {
                            position: [-1.0 * w, 1.0 * h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [1.0 * w, 1.0 * h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [-1.0 * w, -1.0 * h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [1.0 * w, -1.0 * h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color: color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                } else {
                    let vertices = [
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                };

                target
                    .draw(
                        &vb,
                        glium::index::NoIndices(indices),
                        &shaders.shape,
                        &uniforms,
                        &params,
                    )
                    .expect("failed to draw rect");
            }
            Sprite::Circle { .. } => todo!("circles aren't implemented"),
            Sprite::Triangle { w, h, .. } => {
                let w = gj2gl::coord(*w) / 2.0;
                let h = gj2gl::coord(*h) / 2.0;

                let vertices = [
                    Vertex {
                        position: [-w, -h],
                        normal: [-w, -h],
                        color,
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [w, -h],
                        normal: [w, -h],
                        color,
                        tex_coords: [1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, h],
                        normal: [0.0, h],
                        color,
                        tex_coords: [0.5, 1.0],
                    },
                ];

                let vb = VertexBuffer::new(d, &vertices).unwrap();
                target
                    .draw(
                        &vb,
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        &shaders.shape,
                        &uniforms,
                        &params,
                    )
                    .expect("failed to draw triangle");
            }
            Sprite::Text { .. } => todo!("text isn't implemented"),
            Sprite::Texture {
                dimensions,
                w,
                h,
                data,
                ..
            } => {
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&data, *dimensions);
                let texture = glium::Texture2d::new(d, image).unwrap();

                let w = gj2gl::coord(*w) / 2.0;
                let h = gj2gl::coord(*h) / 2.0;

                let vb = if ex.fill {
                    let vertices = [
                        Vertex {
                            position: [-1.0 * w, 1.0 * h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [1.0 * w, 1.0 * h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [-1.0 * w, -1.0 * h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [1.0 * w, -1.0 * h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color: color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                } else {
                    let vertices = [
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color: color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color: color,
                        },
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color: color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                };

                let uniforms = uniforms.add("tex", texture);

                target
                    .draw(
                        &vb,
                        glium::index::NoIndices(indices),
                        &shaders.texture,
                        &uniforms,
                        &params,
                    )
                    .expect("failed to draw texture");
            }
        }
    }

    /// Returns the basic data for the sprite.
    pub fn sprite_data(&self) -> &SpriteData {
        match self {
            Sprite::Rect { ex, .. }
            | Sprite::Circle { ex, .. }
            | Sprite::Triangle { ex, .. }
            | Sprite::Text { ex, .. }
            | Sprite::Texture { ex, .. } => ex,
        }
    }

    /// Returns the basic data for the sprite.
    pub fn sprite_data_mut(&mut self) -> &mut SpriteData {
        match self {
            Sprite::Rect { ex, .. }
            | Sprite::Circle { ex, .. }
            | Sprite::Triangle { ex, .. }
            | Sprite::Text { ex, .. }
            | Sprite::Texture { ex, .. } => ex,
        }
    }
}
