// #![allow(unused)]

use ab::ScaleFont;
use ab_glyph::{self as ab, Font as _};
use harfbuzz_rs as hb;
use image::RgbaImage;
use imageproc::drawing::Canvas as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        "/Users/abdulrahmansibahi/Documents/rust/mimiron/mimiron/fonts/YanoneKaffeesatz-Medium.ttf";
    let font_data = std::fs::read(path)?;

    let index = 0; //< face index in the font file
    let hb_font = hb::Font::new(hb::Face::from_bytes(&font_data, index));

    let buffer = hb::UnicodeBuffer::new().add_str("Hêllo World!");
    let output = hb::shape(&hb_font, buffer, &[]);

    let positions = output.get_glyph_positions();
    let infos = output.get_glyph_infos();

    let ab_font = ab::FontRef::try_from_slice(&font_data)?;

    let width = 300;
    let height = 300;

    let mut canvas: RgbaImage = image::ImageBuffer::new(width, height);
    imageproc::drawing::draw_filled_rect_mut(
        &mut canvas,
        imageproc::rect::Rect::at(0, 0).of_size(width, height),
        image::Rgba([255, 255, 255, 255]),
    );

    let mut caret = 0;

    for (position, info) in positions.iter().zip(infos) {
        let gid = info.codepoint;
        let cluster = info.cluster;
        let x_advance = position.x_advance;
        let x_offset = position.x_offset;
        let y_offset = position.y_offset;

        println!(
            "gid{:?}={:?}@{:?},{:?}+{:?}",
            gid, cluster, x_advance, x_offset, y_offset
        );

        let ab_scale = ab_font.pt_to_px_scale(60.0).unwrap().x;
        let hb_scale = hb_font.scale().0;

        let horizontal = ab_scale * (caret + x_offset) as f32 / hb_scale as f32;
        let vertical =
            ab_font.as_scaled(ab_scale).ascent() + (ab_scale * y_offset as f32 / hb_scale as f32);

        let gl =
            ab_glyph::GlyphId(gid as u16).with_scale_and_position(ab_scale, (horizontal, vertical));

        let Some(y) = ab_font.outline_glyph(gl) else {
            // gl is whitespace
            caret += x_advance;
            continue;
        };

        let bb = y.px_bounds();

        y.draw(|px, py, pv| {
            let px = px as f32 + bb.min.x + 15.0;
            let py = py as f32 + bb.min.y + 15.0;

            let pixel = canvas.get_pixel(px as u32, py as u32).to_owned();
            let color = image::Rgba([0, 0, 0, 255]);
            let weighted_color = imageproc::pixelops::weighted_sum(pixel, color, 1.0 - pv, pv);
            canvas.draw_pixel(px as u32, py as u32, weighted_color);
        });

        caret += x_advance;
    }

    // USELESS BLUE LINE
    for i in 0..15 {
        canvas.draw_pixel(i, i, image::Rgba([0, 0, 255, 255]));
        canvas.draw_pixel(i + 1, i, image::Rgba([0, 0, 255, 255]));
        canvas.draw_pixel(i, i + 1, image::Rgba([0, 0, 255, 255]));
    }

    let save_file = directories::UserDirs::new()
        .expect("couldn't get user directories")
        .desktop_dir()
        .expect("couldn't get downloads directory")
        .to_path_buf()
        .join("fff.png");

    canvas.save(save_file)?;

    Ok(())
}
