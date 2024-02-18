
# NES CHR to Bitmap Converter

This Rust-based NES CHR to Bitmap Converter is specifically designed to extract CHR data from NES games that utilize CHR RAM instead of CHR ROM. In situations where fetching CHR data in conventional ways from a ROM file is challenging, this program provides a convenient solution.

## Purpose

Certain NES games use CHR RAM, complicating the extraction of CHR data. This program simplifies the process by allowing users to extract CHR data from games in this state.

## How to use

The easiest method is to follow these steps:

1. Start the NES game in FCEUX.
2. Navigate to Debug -> Hex Editor and select PPU.
3. Copy one of the pattern tables (0x0000-0x0FFF and 0x1000-0x1FFF) to a text file.
4. Use this program by providing the copied data file in `--chr_file` flag.

> Warning: Pattern tables do not contain information about the palette used for each tile on the screen.

## Usage

```bash
nes-chr-to-bitmap --chr_file <CHR_FILE> --palette <PALETTE> --out_file <OUT_FILE> --palette_type <PALETTE_TYPE>
```

* `--chr_file`: Path to the NES CHR file in text format.
* `--palette`: Palette numbers (space-separated). NES hex palette numbers
* `--out_file`: Output PNG file.
* `--palette_type`: Palette type - There are multiple palettes for different versions of NES. Choose you favourite! (Default: "2C07")
  
## Input File Format

The input CHR file should be in text format. The simplest way to create this file is to open a NES game in FCEUX, access the PPU debug, and copy the 0x0000 to 0x0FFF memory region to a text file.

## Example

```bash
nes-chr-to-bitmap --chr_file path/to/chr_file.txt --palette "0x0D 0x27 0x17 0x06" --out_file path/to/output.png
```

## Notes

* Ensure that the CHR data in the input file is exactly 4096 bytes long.
* The converter processes each tile of 8x8 pixels, so the resulting image will have a size of 128x128 pixels.

## Acknowledgments

This converter is based on resources from [NESdev Wiki](https://www.nesdev.org/wiki/PPU_pattern_tables).

License
This converter is licensed under the MIT License. See the LICENSE file for details.
