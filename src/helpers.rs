#![allow(dead_code)]

use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::graphics::Sprite;

/// Conversions from genji units to glutin/glium units.
pub(crate) mod gj2gl {
    /// Converts a genji coordinate (-400 - 400) to an OpenGL coordinate (-1.0 - 1.0).
    pub fn coord(x: i32) -> f32 {
        // (x as f32 - 500.0) / 1000.0
        x as f32 / 200.0
    }
}

/// Conversions from glutin/glium units to genji units.
pub(crate) mod gl2gj {
    /// Converts a pixel coordinate to a genji coordinate.
    pub fn pxcoord(x: f64, dim: u32) -> i32 {
        ((x / dim as f64 - 0.5 * x.signum()) * 400.0).ceil() as i32
    }
}

/// Helpers for creating and handling matrices.
/// Currently unused.
pub(crate) mod matrix {
    use std::f32::consts::PI;

    /// Create a perspective matrix from screen dimensions.
    /// Currently unused.
    pub fn perspective(dims: (u32, u32)) -> [[f32; 4]; 4] {
        let (width, height) = dims;
        let aspect_ratio = height as f32 / width as f32;

        let fov = PI / 3.0f32;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 1.0],
        ]
    }

    /// Create a view matrix from camera information.
    /// Currently unused.
    pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [
            up[1] * f[2] - up[2] * f[1],
            up[2] * f[0] - up[0] * f[2],
            up[0] * f[1] - up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let p = [
            -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
        ];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

/// Sorts by depth, removing depth-0 (hidden) sprites. Discards id's.
pub(crate) fn sprite_filter<T>(sprites: HashMap<T, Sprite>) -> Vec<Sprite> {
    let mut sprites: Vec<Sprite> = sprites
        .into_iter()
        .filter_map(|(_, s)| {
            if s.sprite_data().depth == 0 {
                None
            } else {
                Some(s)
            }
        })
        .collect();

    sprites.sort_by(|s1, s2| s2.sprite_data().depth.cmp(&s1.sprite_data().depth));

    sprites
}

/// Hashes any type byte-by-byte.
///
/// The &str "abc" and the &[u8] [61, 62, 63] result in the same hash.
pub(crate) fn hash<T>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    let bytes: &[u8] = unsafe {
        core::slice::from_raw_parts((data as *const T) as *const u8, ::core::mem::size_of::<T>())
    };
    for byte in bytes {
        byte.hash(&mut hasher);
    }

    hasher.finish()
}
