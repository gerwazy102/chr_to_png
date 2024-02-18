// App based on following resources:
// https://www.nesdev.org/wiki/PPU_pattern_tables

use std::fs::{self};

use clap::Parser;
use image::{DynamicImage, GenericImage};

mod palette;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Simple NES CHR to BITMAP converter \n\nInput files for this converter should be in txt format. Simplest way is to just open game in FCEUX, open PPU debug and copy 0x0000 to 0x0FFF memory region to txt file.",
    long_about = "Simple NES CHR to BITMAP converter \n\nInput files for this converter should be in txt format. Simplest way is to just open game in FCEUX, open PPU debug and copy 0x0000 to 0x0FFF memory region to txt file."
)]
struct Args {
    /// Path to CHR file
    #[arg(short, long)]
    chr_file: String,

    /// Palette numbers - space separated
    #[arg(short, long)]
    palette: String,

    /// Output png file
    #[arg(short, long)]
    out_file: String,
}

fn main() {
    let args = Args::parse();
    let palette = parse_palette(&args.palette);
    let data = get_file_as_string(&args.chr_file).expect("Failed to read file");
    let nes_chr_bytes = string_data_to_bytes(data).expect("Failed to parse chr txt file");
    if nes_chr_bytes.len() != 0x1000 {
        println!("Input CHR data MUST be exactly 4096 bytes long!");
        println!("Please make sure that you are dumping 0x0000 to 0x0FFF or 0x1000 to 0x1FFF");
    }
    // Here we have vector where each value is one palette color
    let nes_palette_vec = nes_chr_to_palette_vec(nes_chr_bytes);
    let colors = nes_palette_vec_to_colors(nes_palette_vec, palette);
    let mut image = DynamicImage::new_rgb8(128, 128);

    // each tile has 64 pixels as 8x8 so each iteration is one tile
    for i in (0..colors.len()).step_by(64) {
        for y in 0..8 {
            for x in 0..8 {
                let actual_index = i + (y * 8 + x);
                let tile_x = (i / 64) % (image.width() as usize / 8);
                let tile_y = (i / 64) / (image.width() as usize / 8);

                let pixel_x = tile_x * 8 + x;
                let pixel_y = tile_y * 8 + y;
                image.put_pixel(pixel_x as u32, pixel_y as u32, colors[actual_index]);
            }
        }
    }

    image.save(args.out_file).expect("Failed to save image");
}

// Parse a space-separated string into a [u8; 4]
fn parse_palette(palette_str: &str) -> [u8; 4] {
    let mut result = [0; 4];
    let colors: Vec<&str> = palette_str.split_whitespace().collect();

    for (i, color) in colors.iter().take(4).enumerate() {
        // Check if the color starts with "0x" for hexadecimal
        let parsed_color = if color.starts_with("0x") || color.starts_with("0X") {
            u8::from_str_radix(&color[2..], 16)
        } else {
            color.parse()
        };

        result[i] = parsed_color.unwrap_or_else(|_| {
            eprintln!("Invalid palette color: {}", color);
            std::process::exit(1);
        });
    }

    result
}

fn nes_palette_vec_to_colors(nes_palette_vec: Vec<u8>, palette: [u8; 4]) -> Vec<image::Rgba<u8>> {
    let mut pixels: Vec<image::Rgba<u8>> = Vec::with_capacity(128 * 128);
    for &n in nes_palette_vec.iter() {
        pixels.push(palette::nes_to_rgb(palette[n as usize] as usize));
    }

    pixels
}

fn nes_chr_to_palette_vec(chr_bytes: Vec<u8>) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for i in (0..chr_bytes.len()).step_by(16) {
        for j in 0..8 {
            let left_byte = chr_bytes[i + j];
            let right_byte = chr_bytes[i + j + 8];
            let palette_bits = interleave_chr_bits(left_byte, right_byte);
            for i in (0..16).step_by(2) {
                let to_push = ((palette_bits >> 14 - i) & 0b11) as u8;
                buffer.push(to_push);
            }
        }
    }
    buffer
}

fn interleave_chr_bits(bitplane0: u8, bitplane1: u8) -> u16 {
    let mut out = 0u16;
    for i in 0..8 {
        out += (((bitplane1 >> (7 - i)) & 1) as u16) << (15 - 2 * i);
        out += (((bitplane0 >> (7 - i)) & 1) as u16) << (14 - 2 * i);
    }
    out
}

fn string_data_to_bytes(mut data: String) -> Result<Vec<u8>, std::num::ParseIntError> {
    let mut buffer = Vec::new();
    data = data.replace("\n", "");
    let parts: Vec<&str> = data.split_whitespace().collect();

    for hex_str in parts {
        let byte = u8::from_str_radix(hex_str, 16)?;
        buffer.push(byte);
    }
    Ok(buffer)
}

fn get_file_as_string(filename: &String) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interleave_chr_bits() {
        let result1 = interleave_chr_bits(0b00110011, 0b01010101);
        assert_eq!(result1, 0b0010011100100111);

        let result1 = interleave_chr_bits(0b00000000, 0b11111111);
        assert_eq!(result1, 0b1010101010101010);

        let result1 = interleave_chr_bits(0b11111111, 0b00000000);
        assert_eq!(result1, 0b0101010101010101);
    }
    #[test]
    fn test_nes_chr_to_palette_vec() {
        let chr_bytes: Vec<u8> = vec![
            0x41, 0xc2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21,
            0x42, 0x87,
        ];
        let result1 = nes_chr_to_palette_vec(chr_bytes);
        assert_eq!(
            result1,
            [
                0, 1, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 0, 0, 3, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0,
                3, 0, 0, 0, 0, 0, 0, 3, 0, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 2, 0,
                3, 0, 0, 0, 0, 2, 2, 2,
            ]
        );

        let chr_bytes: Vec<u8> = vec![
            0x41, 0xc2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21,
            0x42, 0x87, 0x41, 0xc2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08,
            0x16, 0x21, 0x42, 0x87,
        ];
        let result1 = nes_chr_to_palette_vec(chr_bytes);
        assert_eq!(
            result1,
            [
                0, 1, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 0, 0, 3, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0,
                3, 0, 0, 0, 0, 0, 0, 3, 0, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 2, 0,
                3, 0, 0, 0, 0, 2, 2, 2, 0, 1, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 0, 0, 3, 0, 0, 1, 0, 0,
                0, 3, 0, 0, 0, 1, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 2,
                0, 3, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 2, 2, 2,
            ]
        );
    }
}
