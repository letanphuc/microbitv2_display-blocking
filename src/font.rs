//! A 5×5 ascii font.
//!
//! This is a copy of the 'pendolino' font from the [micro:bit runtime][dal].
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/

mod pendolino;

use microbit::display::nonblocking::GreyscaleImage;

/// Index of the first character in the standard font
pub const PRINTABLE_START: usize = 32;

/// Number of characters in the standard font
pub const PRINTABLE_COUNT: usize = 95;
pub const MAX_BRIGHTNESS: u8 = 9;

const UNKNOWN: GreyscaleImage = GreyscaleImage::new(&[
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
]);

/// Returns an image representing the requested ascii character.
///
/// If the requested character isn't printable, returns a 'hollow square' image.
///
/// # Example
///
/// `font::character(b'x')`
pub fn character(index: u8) -> &'static GreyscaleImage {
    let index = index as usize;
    if !(PRINTABLE_START..PRINTABLE_START + PRINTABLE_COUNT).contains(&index) {
        return &UNKNOWN;
    }
    &pendolino::PENDOLINO3[index - PRINTABLE_START]
}

const fn font_entry(data: [u8; 5]) -> GreyscaleImage {
    // Note the data in the pendolino font uses the opposite column numbering
    // system to BitImage.
    const fn bit_to_brightness(b: u8, index: usize) -> u8 {
        if (b & 1 << index) != 0 {
            MAX_BRIGHTNESS
        } else {
            0
        }
    }
    const fn row_bits(byte: u8) -> [u8; 5] {
        [
            bit_to_brightness(byte, 4),
            bit_to_brightness(byte, 3),
            bit_to_brightness(byte, 2),
            bit_to_brightness(byte, 1),
            bit_to_brightness(byte, 0),
        ]
    }
    GreyscaleImage::new(&[
        row_bits(data[0]),
        row_bits(data[1]),
        row_bits(data[2]),
        row_bits(data[3]),
        row_bits(data[4]),
    ])
}
