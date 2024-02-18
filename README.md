
# NES CHR to Bitmap Converter

This is a simple NES CHR to bitmap converter written in Rust. The converter takes NES CHR data in a text file format and outputs a bitmap image.

## Usage

```bash
nes-chr-to-bitmap --chr_file <CHR_FILE> --palette <PALETTE> --out_file <OUT_FILE>
```

* `--chr_file`: Path to the NES CHR file in text format.
* `--palette`: Palette numbers (space-separated).
* `--out_file`: Output PNG file.
* `--palette_type`: Palette type - There are multiple palettes for different versions of NES. Choose you favourite! (Default: "2C05-99")
  
## Input File Format

The input CHR file should be in text format. The simplest way to create this file is to open a NES game in FCEUX, access the PPU debug, and copy the 0x0000 to 0x0FFF memory region to a text file.

## Example

```bash
nes-chr-to-bitmap --chr_file path/to/chr_file.txt --palette "1 2 3 4" --out_file path/to/output.png
```

## Notes

* Ensure that the CHR data in the input file is exactly 4096 bytes long.
* The converter processes each tile of 8x8 pixels, so the resulting image will have a size of 128x128 pixels.

## Acknowledgments

This converter is based on resources from [NESdev Wiki](https://www.nesdev.org/wiki/PPU_pattern_tables).

License
This converter is licensed under the MIT License. See the LICENSE file for details.
