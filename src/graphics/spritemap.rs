//! Utilities for loading and using spritemaps.
//! 
//! Spritemaps reduce the amount of space and resources
//! required to use a large amount of sprites, and give
//! a convenient holding place for them.
//! 
//! Spritemaps require a default sprite width and
//! height, for the sake of [`Spritemap::get_id`]; however,
//! you can retrieve arbitrarily placed and sized 
//! sprites via [`Spritemap::get_rect`].
//! 
//! Note that retrieving a sprite from a spritemap clones
//! the sprite data, it doesn't reference it.

use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

use image::RgbaImage;

use super::sprite::{self, ImageFormat};

/// A texture with utilities for retrieving sprites.
/// 
/// Spritemaps require a default sprite width and
/// height, for the sake of [`Spritemap::get_id`]; however,
/// you can retrieve arbitrarily placed and sized 
/// sprites via [`Spritemap::get_rect`].
pub struct Spritemap {
    tex: RgbaImage,

    w: u32,
    h: u32,

    sw: u32,
    sh: u32,
}

impl Spritemap {
    /// Creates a new spritemap from image data.
    ///
    /// If the images dimensions do not cleanly divide into
    /// `w` and `h`, returns None.
    pub fn new<D: Into<Vec<u8>>>(data: D, fmt: ImageFormat, w: u32, h: u32) -> Option<Self> {
        let data = image::load(Cursor::new(data.into()), fmt).ok()?.to_rgba8();

        let dims = data.dimensions();

        if dims.0 % w != 0 || dims.1 % h != 0 {
            return None;
        }

        let sw = dims.0 / w;
        let sh = dims.1 / h;

        Some(Self {
            tex: data,

            w,
            h,
            sw,
            sh,
        })
    }

    /// Creates a new spritemap from and image file.
    ///
    /// If the images dimensions do not cleanly divide into
    /// `w` and `h`, returns None.
    pub fn from_file<S: ToString>(path: S, w: u32, h: u32) -> Option<Self> {
        let path = path.to_string();
        let data = image::load(
            BufReader::new(File::open(&path).ok()?),
            ImageFormat::from_extension(
                Path::new(&path)
                    .extension()
                    .map(|e| e.to_str().unwrap_or(""))?,
            )?,
        )
        .ok()?
        .to_rgba8();

        let dims = data.dimensions();

        if dims.0 % w != 0 || dims.1 % h != 0 {
            return None;
        }

        let sw = dims.0 / w;
        let sh = dims.1 / h;

        Some(Self {
            tex: data,

            w,
            h,
            sw,
            sh,
        })
    }

    // TODO: can this be improved?
    fn sample_rect(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
        let mut samples = Vec::new();
        for yy in y..(y + h) {
            for xx in x..(x + w) {
                samples.push((xx, yy));
            }
        }

        let mut pb = Vec::new();
        for (x, y) in samples {
            pb.push(self.tex.get_pixel(x, y).0);
        }

        pb.into_iter().flatten().collect()
    }

    /// Get a sprite using the preset width and height options.
    pub fn get_id(&self, id: u32, w: Option<i32>, h: Option<i32>) -> Option<sprite::Texture> {
        if id > self.sw * self.sh {
            return None;
        }

        let x = (id % self.sw) * self.w;
        let y = (id / self.sw) * self.h;

        let pb = self.sample_rect(x, y, self.w, self.h);
        Some(sprite::texture_raw(pb, (self.w, self.h), w, h))
    }

    /// Get a sub-region of the spritemap, ignoring usual bounds.
    /// 
    /// `tw` and `th` correspond to the `w` and `h` arguments on
    /// [`sprite::texture`].
    pub fn get_rect(&self, x: u32, y: u32, w: u32, h: u32, tw: Option<i32>, th: Option<i32>) -> Option<sprite::Texture> {
        if x+w >= self.dims.0 || y+h >= self.dims.1 {
            return None;
        }

        let pb = self.sample_rect(x, y, w, h);
        Some(sprite::texture_raw(pb, (w, h), tw, th))
    }
}
