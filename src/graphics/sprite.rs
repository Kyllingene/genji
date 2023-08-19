//! The module containing the sprite types and their
//! constructors.
//!
//! The following sprites are available as components:
//! [`Rect`],
//! [`Circle`],
//! [`Triangle`],
//! [`Text`],
//! and [`Texture`].

use std::{
    f32::consts::PI,
    fmt::Debug,
    fs::File,
    io::{BufReader, Cursor, Read},
    path::Path,
};

use super::{shaders, text, Color};

use crate::{
    helpers::gj2gl,
    shape::{Circle, Rect, Triangle},
};

use ab_glyph::FontArc;
use shaders::Shaders;

use glium::{
    implement_vertex, texture::RawImage2d, uniform, Blend, Display, Frame, PolygonMode, Surface,
    VertexBuffer,
};

/// An image format enum for loading images from
/// raw data.
pub use image::ImageFormat;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, color, tex_coords);

/// Used to sort sprites before rendering.
pub(crate) enum Sprite<'a> {
    Rect(&'a Rect),
    Circle(&'a Circle),
    Triangle(&'a Triangle),
    Text(&'a Text),
    Texture(&'a Texture),
}

impl<'a> Sprite<'a> {
    pub(crate) fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        match self {
            Self::Rect(sprite) => sprite.draw(target, ex, d, shaders),
            Self::Circle(sprite) => sprite.draw(target, ex, d, shaders),
            Self::Triangle(sprite) => sprite.draw(target, ex, d, shaders),
            Self::Text(sprite) => sprite.draw(target, ex, d, shaders),
            Self::Texture(sprite) => sprite.draw(target, ex, d, shaders),
        }
    }
}

/// The data required to draw a sprite.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SpriteData {
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

/// A text sprite.
///
/// A font must be passed at creation
/// using a [`FontArc`]
/// from [`ab_glyph`].
/// Alternatively, pass a path to a .otf or .ttf
/// to load a font from a file using
/// `text_font_from_file` instead of `font`.
///
/// ```
/// # use genji::{ecs::World, graphics::Point};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   pub fn text(t: &str, f: (), fs: f32) -> () { () }
/// # }
/// # let font = ();
///
/// world.spawn((
///     sprite::text("", font.clone(), 12.0),
///     Point(0, 0),
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font: FontArc,
    pub font_size: f32,
}

/// A texture sprite.
///
/// You may either pass static data to `texture`,
/// or pass a path to an image file to `texture_from_file`.
/// If using the former, you must pass an [`ImageFormat`]
/// (borrowed from [`image`]).
///
/// ```
/// # use genji::{ecs::World, graphics::{Point, sprite::ImageFormat}};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   use genji::graphics::sprite::ImageFormat;
/// #   pub fn texture(d: (), f: ImageFormat, w: i32, h: i32) -> () { () }
/// # }
/// # let data = ();
///
/// world.spawn((
///     sprite::texture(data, ImageFormat::Png, 300, 300),
///     Point(0, 0),
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Vec<u8>,
    pub dimensions: (u32, u32),
    pub w: i32,
    pub h: i32,
}

/// Creates a [`Text`] from static data.
///
/// A font must be passed at creation using a [`FontArc`]
/// from [`ab_glyph`].
///
/// ```
/// # use genji::{ecs::World, graphics::Point};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   pub fn text(t: &str, f: (), fs: f32) -> () { () }
/// # }
/// # let font = ();
///
/// world.spawn((
///     sprite::text("", font.clone(), 12.0),
///     Point(0, 0),
/// ));
/// ```
pub fn text<S: ToString>(text: S, font_data: &'static [u8], font_size: f32) -> Option<Text> {
    let font = FontArc::try_from_slice(font_data).ok()?;

    Some(Text {
        text: text.to_string(),
        font,
        font_size,
    })
}

/// Creates a [`Text`] with a font file.
///
/// The path must be to a valid .otf / .ttf file.
///
/// ```
/// # use genji::{ecs::World, graphics::Point};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   pub fn text(t: &str, f: (), fs: f32) -> () { () }
/// # }
/// # let font = ();
///
/// world.spawn((
///     sprite::text("", font.clone(), 12.0),
///     Point(0, 0),
/// ));
/// ```
pub fn text_font_from_file<S1: ToString, S2: ToString>(
    text: S1,
    font: S2,
    font_size: f32,
) -> Option<Text> {
    let mut font_file = File::open(font.to_string()).ok()?;
    let mut font_data = Vec::new();
    font_file.read_to_end(&mut font_data).ok()?;

    let font = FontArc::try_from_vec(font_data).ok()?;

    Some(Text {
        text: text.to_string(),
        font,
        font_size,
    })
}

/// Creates a [`Texture`] from binary data.
/// 
/// `w` and `h` work like HTML image dimensions;
/// if only one is specified, the other is scaled to match.
/// If neither, the image keeps a 1px:1coord ratio.
///
/// You must pass an [`ImageFormat`]
/// (borrowed from [`image`]).
///
/// ```
/// # use genji::{ecs::World, graphics::{Point, sprite::ImageFormat}};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   use genji::graphics::sprite::ImageFormat;
/// #   pub fn texture(d: (), f: ImageFormat, w: Option<i32>, h: Option<i32>) -> () { () }
/// # }
/// # let data = ();
///
/// world.spawn((
///     sprite::texture(data, ImageFormat::Png, Some(300), None),
///     Point(0, 0),
/// ));
/// ```
pub fn texture<D>(data: D, fmt: ImageFormat, w: Option<i32>, h: Option<i32>) -> Option<Texture>
where
    D: Into<Vec<u8>>,
{
    let data = data.into();

    let data = image::load(Cursor::new(data), fmt)
        .expect("failed to make tex")
        .to_rgba8();
    // let data = image::load(Cursor::new(data), fmt).ok()?.to_rgba8();

    let dimensions = data.dimensions();

    let (w, h) = match (w, h) {
        (None, None) => (dimensions.0 as i32, dimensions.1 as i32),
        (None, Some(h)) => (
            (dimensions.0 as f32 * (h as f32 / dimensions.1 as f32)).round() as i32,
            h,
        ),
        (Some(w), None) => (
            w,
            (dimensions.1 as f32 * (w as f32 / dimensions.0 as f32)).round() as i32,
        ),
        (Some(w), Some(h)) => (w, h),
    };

    Some(Texture {
        data: data.into_raw(),
        dimensions,
        w,
        h,
    })
}

/// Creates a [`Texture`] from raw pixel data.
/// 
/// `w` and `h` work like HTML image dimensions;
/// if only one is specified, the other is scaled to match.
/// If neither, the image keeps a 1px:1coord ratio.
///
/// Errors in the data will not cause errors here,
/// but rather when the texture is drawn.
///
/// ```
/// # use genji::{ecs::World, graphics::{Point, sprite::ImageFormat}};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   use genji::graphics::sprite::ImageFormat;
/// #   pub fn texture_raw(d: (), dims: (u32, u32), w: Option<i32>, h: Option<i32>) -> () { () }
/// # }
/// # let data = ();
///
/// world.spawn((
///     sprite::texture_raw(data, (200, 150), Some(300), None),
///     Point(0, 0),
/// ));
/// ```
pub fn texture_raw<D>(data: D, dimensions: (u32, u32), w: Option<i32>, h: Option<i32>) -> Texture
where
    D: Into<Vec<u8>>,
{
    let data = data.into();

    let (w, h) = match (w, h) {
        (None, None) => (dimensions.0 as i32, dimensions.1 as i32),
        (None, Some(h)) => (
            (dimensions.0 as f32 * (h as f32 / dimensions.1 as f32)).round() as i32,
            h,
        ),
        (Some(w), None) => (
            w,
            (dimensions.1 as f32 * (w as f32 / dimensions.0 as f32)).round() as i32,
        ),
        (Some(w), Some(h)) => (w, h),
    };

    Texture {
        data,
        dimensions,
        w,
        h,
    }
}

/// Creates a [`Texture`] from an image file.
/// 
/// `w` and `h` work like HTML image dimensions;
/// if only one is specified, the other is scaled to match.
/// If neither, the image keeps a 1px:1coord ratio.
///
/// ```
/// # use genji::{ecs::World, graphics::{Point, sprite::ImageFormat}};
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # mod sprite {
/// #   use genji::graphics::sprite::ImageFormat;
/// #   pub fn texture_from_file(path: (), w: Option<i32>, h: Option<i32>) -> () { () }
/// # }
/// # let path = ();
///
/// world.spawn((
///     sprite::texture_from_file(path, Some(300), None),
///     Point(0, 0),
/// ));
/// ```
pub fn texture_from_file<S: ToString>(path: S, w: Option<i32>, h: Option<i32>) -> Option<Texture> {
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

    let (w, h) = match (w, h) {
        (None, None) => (dimensions.0 as i32, dimensions.1 as i32),
        (None, Some(h)) => (
            (dimensions.0 as f32 * (h as f32 / dimensions.1 as f32)).round() as i32,
            h,
        ),
        (Some(w), None) => (
            w,
            (dimensions.1 as f32 * (w as f32 / dimensions.0 as f32)).round() as i32,
        ),
        (Some(w), Some(h)) => (w, h),
    };

    Some(Texture {
        data: data.into_raw(),
        dimensions,
        w,
        h,
    })
}

pub(crate) trait DrawSprite {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders);
}

impl DrawSprite for Rect {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        let mut params = glium::DrawParameters {
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

        let w = gj2gl::coord(self.w) / 2.0;
        let h = gj2gl::coord(self.h) / 2.0;

        let vb = if ex.fill {
            let vertices = [
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                    color,
                },
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                    color,
                },
            ];

            VertexBuffer::new(d, &vertices).unwrap()
        } else {
            let vertices = [
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                    color,
                },
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                Vertex {
                    position: [-w, h],
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
}

impl DrawSprite for Circle {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        let mut params = glium::DrawParameters {
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

        let r = gj2gl::coord(self.r);
        let mut vertices = Vec::new();

        let mut a = 0.0f32;
        while a <= 360.0 {
            let pos = [r * a.cos(), r * a.sin()];
            vertices.push(Vertex {
                position: pos,
                color,
                tex_coords: [pos[0] + 0.5, pos[1] + 0.5],
            });

            if ex.fill && a % 1.0 == 0.0 {
                vertices.push(Vertex {
                    position: [0.0, 0.0],
                    color,
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
}

impl DrawSprite for Triangle {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        let params = glium::DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        let color = ex.color.to_f32();

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

        let w = gj2gl::coord(self.w) / 2.0;
        let h = gj2gl::coord(self.h) / 2.0;
        let o = gj2gl::coord(self.o);

        let vertices = [
            Vertex {
                position: [-w, -h],
                color,
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [w, -h],
                color,
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [o, h],
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
}

impl DrawSprite for Text {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        let mut params = glium::DrawParameters {
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

        let (buf, w, h) = text::render_glyphs(&self.font, self.font_size, &self.text, &ex);

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
                    tex_coords: [0.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                    color,
                },
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                    color,
                },
            ],
        )
        .unwrap();

        let uniforms = uniform! {
            matrix: mat,
            tex: texture,
        };

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

impl DrawSprite for Texture {
    fn draw(&self, target: &mut Frame, ex: SpriteData, d: &Display, shaders: &Shaders) {
        let mut params = glium::DrawParameters {
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

        let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&self.data, self.dimensions);
        let texture = glium::Texture2d::new(d, raw).unwrap();

        let w = gj2gl::coord(self.w) / 2.0;
        let h = gj2gl::coord(self.h) / 2.0;

        let vb = if ex.fill {
            let vertices = [
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                    color,
                },
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                    color,
                },
            ];

            VertexBuffer::new(d, &vertices).unwrap()
        } else {
            let vertices = [
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, h],
                    tex_coords: [1.0, 1.0],
                    color,
                },
                Vertex {
                    position: [w, -h],
                    tex_coords: [1.0, 0.0],
                    color,
                },
                Vertex {
                    position: [-w, -h],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                Vertex {
                    position: [-w, h],
                    tex_coords: [0.0, 1.0],
                    color,
                },
            ];

            VertexBuffer::new(d, &vertices).unwrap()
        };

        let uniforms = uniform! {
            matrix: mat,
            tex: texture,
        };

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
