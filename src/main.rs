use harfbuzz_rs as hb;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "YanoneKaffeesatz-Medium.ttf";
    // let path = "NotoNaskhArabic-Bold.ttf";
    // let path = "Gulzar-Regular.ttf";

    let buffer = hb::UnicodeBuffer::new().add_str("Hêllo World!");
    // let buffer = hb::UnicodeBuffer::new().add_str("أهلا بالعالم");

    let font_data = std::fs::read(path)?;

    let index = 0; //< face index in the font file
    let hb_font = hb::Font::new(hb::Face::from_bytes(&font_data, index));

    let output = hb::shape(&hb_font, buffer, &[]);

    let positions = output.get_glyph_positions();
    let infos = output.get_glyph_infos();

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
    }

    Ok(())
}
