use std::{
    f32::consts::PI,
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

use crate::helpers::gj2gl;

mod text;

pub(crate) mod shaders;
use ab_glyph::FontArc;
use shaders::Shaders;

use glium::{
    implement_vertex, texture::RawImage2d, uniform, Blend, Display, Frame, PolygonMode, Surface,
    VertexBuffer,
};

pub use image::ImageFormat;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    normal: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, color, tex_coords);

/// An RGBA color in byte format.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// The red channel of the color.
    pub r: u8,
    /// The green channel of the color.
    pub g: u8,
    /// The blue channel of the color.
    pub b: u8,
    /// The opacity of the color.
    pub a: u8,
}

impl Color {
    /// Creates an opaque white color. Use the builder
    /// pattern to adjust the color.
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Sets the red channel of the color.
    pub fn r(mut self, r: u8) -> Self {
        self.r = r;
        self
    }

    /// Sets the green channel of the color.
    pub fn g(mut self, g: u8) -> Self {
        self.g = g;
        self
    }

    /// Sets the blue channel of the color.
    pub fn b(mut self, b: u8) -> Self {
        self.b = b;
        self
    }

    /// Sets the opacity of the color.
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
    /// Creates a new sprite with default values.
    /// See each property to see defaults.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the horizontal position of the sprite.
    /// Defaults to `0`.
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    /// Sets the vertical position of the sprite.
    /// Defaults to `0`.
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    /// Sets the position of the sprite.
    /// Defaults to `0, 0`.
    pub fn xy(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the z-level of the sprite. `0` hides it.
    /// Defaults to `1`.
    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    /// Sets the rotation of the sprite, in degrees.
    /// Defaults to `0.0`.
    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    /// Sets whether or not to fill the sprite (only for shapes).
    /// Defaults to `true`.
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Sets the weight of the lines if `!fill` (only for shapes).
    /// Defaults to `4`.
    pub fn stroke_weight(mut self, stroke_weight: u32) -> Self {
        self.stroke_weight = stroke_weight;
        self
    }

    /// Sets the color of the sprite (for sprites, offsets the color).
    /// Defaults to opaque white.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
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
        text: String,
        font: FontArc,
        font_size: f32,
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

// TODO: probably ditch current sprite system
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

    pub fn text<S: ToString>(
        text: S,
        font_data: &'static [u8],
        font_size: f32,
        ex: SpriteData,
    ) -> Option<Self> {
        let font = FontArc::try_from_slice(font_data).ok()?;

        Some(Self::Text {
            text: text.to_string(),
            font,
            font_size,
            ex,
        })
    }

    pub fn text_font_file<S1: ToString, S2: ToString>(
        text: S1,
        font: S2,
        font_size: f32,
        ex: SpriteData,
    ) -> Option<Self> {
        let mut font_file = File::open(font.to_string()).ok()?;
        let mut font_data = Vec::new();
        font_file.read_to_end(&mut font_data).ok()?;

        let font = FontArc::try_from_vec(font_data).ok()?;

        Some(Self::Text {
            text: text.to_string(),
            font,
            font_size,
            ex,
        })
    }

    pub fn texture<S: ToString>(path: S, w: i32, h: i32, ex: SpriteData) -> Option<Self> {
        let path = path.to_string();
        let data = image::load(
            BufReader::new(File::open(&path).ok()?),
            image::ImageFormat::from_extension(
                Path::new(&path)
                    .extension()
                    .map(|e| e.to_str().unwrap_or(""))?,
            )?,
        )
        .ok()?
        .to_rgba8();

        let dimensions = data.dimensions();

        Some(Self::Texture {
            path,
            data: data.into_raw(),
            dimensions,
            w,
            h,
            ex,
        })
    }

    pub fn texture_from_raw<S, D>(
        path: Option<S>,
        data: D,
        fmt: Option<ImageFormat>,
        w: i32,
        h: i32,
        ex: SpriteData,
    ) -> Option<Self>
    where
        S: ToString,
        D: Into<Vec<u8>>,
    {
        let data = data.into();
        let path = path.map(|s| s.to_string());

        let data = image::load(
            Cursor::new(data),
            fmt.unwrap_or(image::ImageFormat::from_extension(
                Path::new(path.as_ref()?)
                    .extension()
                    .map(|e| e.to_str().unwrap_or(""))?,
            )?),
        )
        .ok()?
        .to_rgba8();

        let dimensions = data.dimensions();

        Some(Self::Texture {
            path: path.unwrap_or_default(),
            data: data.into_raw(),
            dimensions,
            w,
            h,
            ex,
        })
    }

    pub(crate) fn draw(&self, target: &mut Frame, d: &Display, shaders: &Shaders) {
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
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                } else {
                    let vertices = [
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
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
            Sprite::Circle { r, ex } => {
                let r = gj2gl::coord(*r);
                let mut vertices = Vec::new();

                let mut a = 0.0f32;
                while a <= 360.0 {
                    let pos = [r * a.cos(), r * a.sin()];
                    let sum = pos[0] + pos[1];
                    vertices.push(Vertex {
                        position: pos,
                        normal: [pos[0] / sum, pos[1] / sum],
                        color: ex.color.to_f32(),
                        tex_coords: [pos[0] + 0.5, pos[1] + 0.5],
                    });

                    if ex.fill && a % 1.0 == 0.0 {
                        vertices.push(Vertex {
                            position: [0.0, 0.0],
                            normal: [1.0, 0.0],
                            color: ex.color.to_f32(),
                            tex_coords: [0.5, 0.5],
                        });
                    }

                    a += 0.5;
                }

                let vb = VertexBuffer::new(d, &vertices).unwrap();

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
            Sprite::Text {
                text,
                font,
                font_size,
                ex,
            } => {
                let (buf, w, h) = text::render_glyphs(font, *font_size, text, ex);

                let raw = RawImage2d::from_raw_rgba_reversed(
                    buf.into_iter()
                        .flatten()
                        .flat_map(|(r, g, b, a)| [r, g, b, a])
                        .collect::<Vec<_>>()
                        .as_slice(),
                    (w as u32, h as u32),
                );

                let texture = glium::Texture2d::new(d, raw).unwrap();

                // Scaling down the mesh forces the font size to get bigger,
                // which results in higher quality textures and less blur.
                let w = gj2gl::coord(w as i32) * 0.5;
                let h = gj2gl::coord(h as i32) * 0.5;
                let vb = VertexBuffer::new(
                    d,
                    &[
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color,
                        },
                    ],
                )
                .unwrap();

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
            Sprite::Texture {
                dimensions,
                w,
                h,
                data,
                ..
            } => {
                let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(data, *dimensions);
                let texture = glium::Texture2d::new(d, raw).unwrap();

                let w = gj2gl::coord(*w) / 2.0;
                let h = gj2gl::coord(*h) / 2.0;

                let vb = if ex.fill {
                    let vertices = [
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color,
                        },
                    ];

                    VertexBuffer::new(d, &vertices).unwrap()
                } else {
                    let vertices = [
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, h],
                            normal: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                            color,
                        },
                        Vertex {
                            position: [w, -h],
                            normal: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [-w, -h],
                            normal: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                            color,
                        },
                        Vertex {
                            position: [-w, h],
                            normal: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                            color,
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
