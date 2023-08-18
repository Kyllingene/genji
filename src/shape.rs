//! A collection of geometric utilities.
//!
//! Provides the primitive shapes [`Rect`],
//! [`Circle`], and [`Triangle`], as well as
//! the point-inclusion trait [`Contains`] and
//! implementations for all geometric sprites.
//!
//! Also provides the foundational [`Point`]
//! struct, which is a basic 2D vector.

use std::ops::{Add, Div, Mul, Sub};

use crate::graphics::sprite::Texture;

// TODO: implement full collision detection

/// A 2D vector, usually positional.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn<T>(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
/// # fn some_sprite() {}
///
/// world.spawn((
///     some_sprite(),
///     Point(25, 25),
/// ));
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    /// Returns the length of the vector.
    pub fn len(&self) -> f32 {
        ((self.0 as f32).powi(2) + (self.1 as f32).powi(2)).sqrt()
    }

    /// Returns a normalized vector.
    ///
    /// Returns an `(f32, f32)`, since normalization with integers
    /// doesn't work so well.
    pub fn norm(&self) -> (f32, f32) {
        let len = self.len();
        (self.0 as f32 / len, self.1 as f32 / len)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.0 > self.0 && other.1 > self.1 {
            Some(std::cmp::Ordering::Less)
        } else if other.0 < self.0 && other.1 < self.1 {
            Some(std::cmp::Ordering::Greater)
        } else if other.0 == self.0 && other.1 == self.1 {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul for Point {
    type Output = i32;

    /// The cross product of two vectors.
    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}

fn pivot(point: Point, angle: f32, pivot: Point) -> Point {
    let angle = angle.to_radians();

    Point(
        (((point.0 - pivot.0) as f32) * angle.cos() - ((point.1 - pivot.1) as f32) * angle.sin())
            .round() as i32
            + pivot.0,
        (((point.0 - pivot.0) as f32) * angle.sin() + ((point.1 - pivot.1) as f32) * angle.cos())
            .round() as i32
            + pivot.1,
    )
}

fn orientation(a: Point, b: Point, c: Point) -> bool {
    let ab = b - a;
    let ac = c - a;
    ab * ac > 0
}

fn triangle_points(pos: Point, mut w: i32, mut h: i32, o: i32) -> (Point, Point, Point) {
    w /= 2;
    h /= 2;

    (
        Point(pos.0 - w, pos.1 - h),
        Point(pos.0 + w, pos.1 - h),
        Point(pos.0 + o, pos.1 + h),
    )
}

pub trait Contains {
    fn contains(&self, pos: Point, point: Point, angle: f32) -> bool {
        self.contains_corrected(pos, pivot(point, angle, pos))
    }

    fn contains_corrected(&self, pos: Point, point: Point) -> bool;
}

impl Contains for Circle {
    fn contains_corrected(&self, pos: Point, point: Point) -> bool {
        (pos - point).len() < self.r as f32
    }
}

impl Contains for Rect {
    fn contains_corrected(&self, pos: Point, point: Point) -> bool {
        let min = Point(pos.0 - (self.w / 2), pos.1 - (self.h / 2));
        let max = Point(pos.0 + (self.w / 2), pos.1 + (self.h / 2));

        min.0 <= point.0 && point.0 <= max.0 && min.1 <= point.1 && point.1 <= max.1
    }
}

impl Contains for Triangle {
    fn contains_corrected(&self, pos: Point, point: Point) -> bool {
        let me = triangle_points(pos, self.w, self.h, self.o);

        orientation(me.0, me.1, point)
            && orientation(me.1, me.2, point)
            && orientation(me.2, me.0, point)
    }
}

impl Contains for Texture {
    fn contains_corrected(&self, pos: Point, point: Point) -> bool {
        let min = Point(pos.0 - (self.w / 2), pos.1 - (self.h / 2));
        let max = Point(pos.0 + (self.w / 2), pos.1 + (self.h / 2));

        min < point && point < max
    }
}

/// A rectangle shape.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     shape::rect(12, 34),
///     Point(0, 0),
/// ));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub w: i32,
    pub h: i32,
}

/// A circle shape.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     shape::circle(30),
///     Point(0, 0),
/// ));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub r: i32,
}

/// A triangle shape.
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     shape::triangle(
///         12, // width of the base
///         34, // height from base -> tip
///         8,  // horizontal offset of tip
///     ),
///     Point(0, 0),
/// ));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub w: i32,
    pub h: i32,
    pub o: i32,
}

/// Creates a [`Rect`].
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     sprite::rect(12, 34),
///     Point(0, 0),
/// ));
/// ```
pub fn rect(w: i32, h: i32) -> Rect {
    Rect { w, h }
}

/// Creates a [`Circle`].
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     sprite::circle(30),
///     Point(0, 0),
/// ));
/// ```
pub fn circle(r: i32) -> Circle {
    Circle { r }
}

/// Creates a [`Triangle`].
///
/// ```
/// # use genji::prelude::*;
/// # struct FakeWorld;
/// # impl FakeWorld {
/// #   pub fn spawn(&self, x: T) {}
/// # }
/// # let world = FakeWorld;
///
/// world.spawn((
///     sprite::triangle(
///         12, // width of the base
///         34, // height from base -> tip
///         8,  // horizontal offset of tip
///     ),
///     Point(0, 0),
/// ));
/// ```
pub fn triangle(w: i32, h: i32, o: i32) -> Triangle {
    Triangle { w, h, o }
}
