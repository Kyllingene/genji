//! Utilities for drawing things to the screen.
//!
//! In genji, sprites are components that can optionally
//! have other components attached to add information. The
//! exception is [`Point`](crate::shape::Point),
//! which must be specified or the sprite will not be drawn.
//! All the others have default values.
//!
//! The following sprites are available as components:
//! [`Rect`](crate::shape::Rect),
//! [`Circle`](crate::shape::Circle),
//! [`Triangle`](crate::shape::Triangle),
//! [`Text`](sprite::Text),
//! and [`Texture`](sprite::Texture).
//!
//! Data can be attached to sprites via several components:
//! [`Angle`],
//! [`Color`],
//! [`Depth`],
//! [`Fill`],
//! [`Point`](crate::shape::Point),
//! [`StrokeWeight`].

use std::ops::{Deref, DerefMut};

pub(crate) mod shaders;
pub mod sprite;
pub mod spritemap;
mod text;

/// An RGBA color in byte format.
///
/// Defaults to opaque white.
///
/// ```
/// # use genji::graphics::Color;
///
/// let color1 = Color::new(12, 34, 56, 255);
/// let color2 = Color::default()
///     .r(12)
///     .g(34)
///     .b(56);
///
/// assert_eq!(color1, color2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// A sprites depth. `0` hides the sprite.
///
/// Defaults to `1`.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # fn some_sprite() -> () { () }
///
/// world.spawn((
///     some_sprite(),
///     Depth(123),
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(pub u32);

impl Deref for Depth {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Depth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The angle of a sprite.
///
/// Defaults to `0.0`.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # fn some_sprite() -> () { () }
///
/// world.spawn((
///     some_sprite(),
///     Angle(32.3),
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f32);

impl Deref for Angle {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Angle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Whether or not to fill the sprite.
/// Doesn't apply to textures or text.
///
/// Defaults to `true`.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # fn some_sprite() -> () { () }
///
/// world.spawn((
///     some_sprite(),
///     Fill(false),
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fill(pub bool);

impl Deref for Fill {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fill {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The weight of the lines if `!fill` (only for shapes).
///
/// Defaults to `4`.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # fn some_sprite() -> () { () }
///
/// world.spawn((
///     some_sprite(),
///     StrokeWeight(8),
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrokeWeight(pub u32);

impl Deref for StrokeWeight {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrokeWeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
