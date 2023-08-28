//! The things you're most likely to need when using Genji.

/// Includes a file as a byte slice. Equivalent to
/// `pub const _: &[u8] = include_bytes!(_);`.
///
/// ```ignore
/// // pub const NAME: &[u8] = include_bytes!("path/to/file.jpg");
/// use_file!(NAME: "path/to/file.jpg");
/// ```
#[macro_export]
macro_rules! use_file {
    ($name:ident: $path:expr) => {
        pub const $name: &[u8] = include_bytes!($path);
    };
}

/// Include multiple files at once. Equivalent to using
/// `use_file!` on each file.
///
/// ```ignore
/// // pub const NAME_ONE: &[u8] = include_bytes!("path/to/file.jpg");
/// // pub const NAME_TWO: &[u8] = include_bytes!("other-path-to/file.ttf");
/// use_files!{
///     NAME_ONE: "path/to/file.jpg",
///     NAME_TWO: "../other-path/to/file.ttf"
/// }
/// ```
#[macro_export]
macro_rules! use_files {
    ($($name:ident: $path:expr),*) => {
        $(
            pub const $name: &[u8] = include_bytes!($path);
        )*
    };

    ($($name:ident: $path:expr),* ,) => {
        $(
            pub const $name: &[u8] = include_bytes!($path);
        )*
    };
}

pub use crate::{
    audio::{Audio, MusicStore, Sound, SoundSettings, SoundStore},
    ecs::{Entity, World},
    graphics::{
        sprite::{self, ImageFormat},
        spritemap::Spritemap,
        Angle, Color, Depth, Fill, StrokeWeight,
    },
    input::Key,
    shape::{self, Circle, Contains, Point, Rect, Triangle},
    state::GameState,
    use_file, use_files,
};
