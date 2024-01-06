// #![allow(unused)]

use ab_glyph::{self as ab, Font as _, ScaleFont as _};
use harfbuzz_rs as hb;
use imageproc::drawing::Canvas as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = "YanoneKaffeesatz-Medium.ttf";
    // let path = "NotoNaskhArabic-Bold.ttf";
    let path = "Gulzar-Regular.ttf";

    // let buffer = hb::UnicodeBuffer::new().add_str("Hêllo World!");
    let buffer = hb::UnicodeBuffer::new().add_str("أهلا بالعالم");

    let font_data = std::fs::read(path)?;

    let index = 0; //< face index in the font file
    let hb_font = hb::Font::new(hb::Face::from_bytes(&font_data, index));

    let output = hb::shape(&hb_font, buffer, &[]);

    let positions = output.get_glyph_positions();
    let infos = output.get_glyph_infos();

    let ab_font = ab::FontRef::try_from_slice(&font_data)?;

    let width = 1000;
    let height = 300;

    let mut canvas: image::RgbaImage = image::ImageBuffer::new(width, height);
    imageproc::drawing::draw_filled_rect_mut(
        &mut canvas,
        imageproc::rect::Rect::at(0, 0).of_size(width, height),
        image::Rgba([255, 255, 255, 255]),
    );

    let mut caret = 0;

    let ab_scale = ab_font.pt_to_px_scale(60.0).unwrap();
    let hb_scale = hb_font.scale();

    dbg!(ab_scale);
    dbg!(hb_font.scale());

    let conversion = ab_scale.x / hb_scale.0 as f32;

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

        let horizontal = conversion * (caret + x_offset) as f32;
        let vertical = conversion * -y_offset as f32 +
            // to push the line down.
            ab_font.as_scaled(ab_scale).ascent();

        let gl = ab_glyph::GlyphId(gid as u16)
            .with_scale_and_position(ab_scale, ab_glyph::point(horizontal, vertical));

        let Some(outlined_glyph) = ab_font.outline_glyph(gl) else {
            // gl is whitespace
            caret += x_advance;
            continue;
        };

        let bb = outlined_glyph.px_bounds();

        // USELESS BLUE BOUNDING BOX
        for x in bb.min.x as u32..=bb.max.x as u32 {
            canvas.draw_pixel(x + 15, bb.min.y as u32 + 15, image::Rgba([0, 0, 255, 255]));
            canvas.draw_pixel(x + 15, bb.max.y as u32 + 15, image::Rgba([0, 0, 255, 255]));
        }
        for y in bb.min.y as u32..=bb.max.y as u32 {
            canvas.draw_pixel(bb.min.x as u32 + 15, y + 15, image::Rgba([0, 0, 255, 255]));
            canvas.draw_pixel(bb.max.x as u32 + 15, y + 15, image::Rgba([0, 0, 255, 255]));
        }

        outlined_glyph.draw(|px, py, pv| {
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
