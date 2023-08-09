use ab_glyph::{point, Font, FontArc, Glyph, Point};
use glium::{pixel_buffer::PixelBuffer, Display, Texture2d};

use crate::helpers::gj2gl;

use super::SpriteData;

pub fn chtt(
    ch: char,
    font: &FontArc,
    ex: &SpriteData,
    d: &Display,
) -> Option<(Texture2d, f32, f32)> {
    let glyph: Glyph = font
        .glyph_id(ch)
        .with_scale_and_position(64.0, point(0.0, 0.0));
    // let bounds = font.glyph_bounds(&q_glyph);
    let outlined_glyph = font.outline_glyph(glyph)?;

    let bounds = outlined_glyph.px_bounds();
    let Point {
        x: ch_width,
        y: ch_height,
    } = bounds.max - bounds.min;

    let width = ch_width.ceil() as usize + 2;
    let height = ch_height.ceil() as usize + 2;

    let pxb: PixelBuffer<_> = PixelBuffer::new_empty(d, width * height);
    let mut buf = vec![(0u8, 0u8, 0u8, 0u8); width * height];

    let color = ex.color.to_f32();
    outlined_glyph.draw(|x, y, c| {
        buf[((y + 1) as usize * width) + x as usize + 1] = (
            (c * color[0] * 255.0) as u8,
            (c * color[1] * 255.0) as u8,
            (c * color[2] * 255.0) as u8,
            (c * color[3] * 255.0) as u8,
        );
    });

    pxb.write(&buf);
    let tex: Vec<(u8, u8, u8, u8)> = pxb.read_as_texture_1d().ok()?;
    let tex = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &tex.into_iter()
            .flat_map(|(r, g, b, a)| [r, g, b, a])
            .collect::<Vec<_>>(),
        (width as u32, height as u32),
    );

    Texture2d::new(d, tex).ok().map(|ch| (
        ch,
        gj2gl::coord(ch_width as i32) * 8.0,
        gj2gl::coord(ch_height as i32) * 8.0,
    ))
}
