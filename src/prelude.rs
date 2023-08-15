/// Includes a file as a byte slice. Equivalent to
/// `const _: &[u8] = include_bytes!(_);`.
///
/// ```ignore
/// // const NAME: &[u8] = include_bytes!("path/to/file.jpg");
/// use_file!(NAME: "path/to/file.jpg");
/// ```
#[macro_export]
macro_rules! use_file {
    ($name:ident: $path:expr) => {
        const $name: &[u8] = include_bytes!($path);
    };
}

/// Include multiple files at once. Equivalent to using
/// `use_file!` on each file.
///
/// ```ignore
/// // const NAME_ONE: &[u8] = include_bytes!("path/to/file.jpg");
/// // const NAME_TWO: &[u8] = include_bytes!("other-path-to/file.ttf");
/// use_files!{
///     NAME_ONE: "path/to/file.jpg",
///     NAME_TWO: "../other-path/to/file.ttf"
/// }
/// ```
#[macro_export]
macro_rules! use_files {
    ($($name:ident: $path:expr),*) => {
        $(
            const $name: &[u8] = include_bytes!($path);
        )*
    };
}

pub use crate::{
    ecs::World,
    graphics::{sprite, Angle, Color, Depth, Fill, Position, StrokeWeight},
    input::Key,
    state::GameState,
    use_file, use_files,
};
