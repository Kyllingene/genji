use ab_glyph::{point, Font, FontArc, Glyph, Point, PxScale, ScaleFont};

use super::SpriteData;

fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}

pub fn render_glyphs(
    font: &FontArc,
    font_size: f32,
    text: &str,
    ex: &SpriteData,
) -> (Vec<Vec<(u8, u8, u8, u8)>>, usize, usize) {
    let scale = PxScale::from(font_size);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 9999.0, text, &mut glyphs);

    let glyphs_height = scaled_font.height().ceil() as usize + 50;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as usize
    } + 50;

    let mut buf = vec![vec![(0u8, 0u8, 0u8, 0u8); glyphs_width]; glyphs_height];

    let color = ex.color.to_f32();
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                // let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // // Turn the coverage into an alpha value (blended with any previous)
                // *px = Rgba([
                //     colour.0,
                //     colour.1,
                //     colour.2,
                //     px.0[3].saturating_add((v * 255.0) as u8),
                // ]);
                buf[y as usize + bounds.min.y as usize][x as usize + bounds.min.x as usize] = (
                    ex.color.r,
                    ex.color.g,
                    ex.color.b,
                    (color[3] * v * 382.5).clamp(0.0, 255.0) as u8,
                );
            });
        }
    }

    (buf, glyphs_width, glyphs_height)
}
