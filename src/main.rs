use std::path::PathBuf;

use clap::Parser;
use harfbuzz_rs::{shape, Face, Font, GlyphInfo, GlyphPosition, UnicodeBuffer};

/// Randomly test the shaping of a reference font against another.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Font to use as the reference.
    #[clap(short, long)]
    reference_font: PathBuf,

    /// Font to compare against.
    #[clap(short, long)]
    other_font: PathBuf,
}

#[derive(Debug, PartialEq)]
struct GlyphShaping {
    gid: u32,
    cluster: u32,
    x_advance: i32,
    x_offset: i32,
    y_offset: i32,
}

impl From<(&GlyphInfo, &GlyphPosition)> for GlyphShaping {
    fn from((info, position): (&GlyphInfo, &GlyphPosition)) -> Self {
        Self {
            gid: info.codepoint,
            cluster: info.cluster,
            x_advance: position.x_advance,
            x_offset: position.x_offset,
            y_offset: position.y_offset,
        }
    }
}

fn main() {
    let args = Args::parse();

    let face_reference =
        Face::from_file(args.reference_font, 0).expect("Cannot open reference font.");
    let font_reference = Font::new(face_reference);

    let face_other = Face::from_file(args.other_font, 0).expect("Cannot open other font.");
    let font_other = Font::new(face_other);

    let mut scratch_string = String::new();
    for c1 in 0x20 as char..=0x07E as char {
        for c2 in 0x20 as char..=0x07E as char {
            scratch_string.push(c1);
            scratch_string.push(c2);

            let buffer_reference = UnicodeBuffer::new().add_str(&scratch_string);
            let output_reference = shape(&font_reference, buffer_reference, &[]);
            let positions_reference = output_reference.get_glyph_positions();
            let infos_reference = output_reference.get_glyph_infos();
            let shaping_reference: Vec<GlyphShaping> = infos_reference
                .iter()
                .zip(positions_reference)
                .map(|v| v.into())
                .collect();

            let buffer_other = UnicodeBuffer::new().add_str(&scratch_string);
            let output_other = shape(&font_other, buffer_other, &[]);
            let positions_other = output_other.get_glyph_positions();
            let infos_other = output_other.get_glyph_infos();
            let shaping_other: Vec<GlyphShaping> = infos_other
                .iter()
                .zip(positions_other)
                .map(|v| v.into())
                .collect();

            if shaping_reference != shaping_other {
                println!("Different shaping for '{scratch_string}'")
            }

            scratch_string.clear();
        }
    }
}
