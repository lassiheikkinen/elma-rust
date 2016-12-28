#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![deny(missing_docs)]

//! Library for reading and writing Elasto Mania files.

extern crate byteorder;
extern crate rand;

use std::{io, str, string};
use std::ascii::AsciiExt;

/// Read and write Elasto Mania level files.
pub mod lev;
/// Read and write Elasto Mania replay files.
pub mod rec;

/// General errors.
#[derive(Debug, PartialEq)]
pub enum ElmaError {
    /// Across files are not supported.
    AcrossUnsupported,
    /// Not a level file.
    InvalidLevelFile,
    /// Invalid gravity value.
    InvalidGravity(i32),
    /// Invalid object value.
    InvalidObject(i32),
    /// Invalid clipping value.
    InvalidClipping(i32),
    /// End-of-data marker mismatch.
    EODMismatch,
    /// End-of-file marker mismatch.
    EOFMismatch,
    /// Invalid event value.
    InvalidEvent(u8),
    /// End-of-replay marker mismatch.
    EORMismatch,
    /// Invalid time format.
    InvalidTimeFormat,
    /// Too short padding.
    PaddingTooShort(isize),
    /// String contains non-ASCII characters.
    NonASCII,
    /// Input/output errors from std::io use.
    Io(std::io::ErrorKind),
    /// String errors from std::String.
    StringFromUtf8(usize),
}

impl From<io::Error> for ElmaError {
    fn from(err: io::Error) -> ElmaError {
        ElmaError::Io(err.kind())
    }
}

impl From<string::FromUtf8Error> for ElmaError {
    fn from(err: string::FromUtf8Error) -> ElmaError {
        ElmaError::StringFromUtf8(err.utf8_error().valid_up_to())
    }
}

/// Shared position struct used in both sub-modules.
///
/// # Examples
/// ```
/// let vertex = elma::Position { x: 23.1928_f64, y: -199.200019_f64 };
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct Position<T> {
    /// X-position.
    pub x: T,
    /// Y-position.
    pub y: T
}

/// Trims trailing bytes after and including null byte.
///
/// # Examples
/// As all strings in Elma files are C-strings with padded null-bytes, you can use this function
/// to remove null-bytes and any potential garbage data follwing it and return a String.
///
/// ```
/// let cstring: [u8; 10] = [0x45, 0x6C, 0x6D, 0x61, 0x00, 0x00, 0x00, 0x7E, 0x7E, 0x7E];
/// let trimmed = elma::trim_string(&cstring).unwrap();
/// assert_eq!(trimmed, "Elma");
/// ```
pub fn trim_string (data: &[u8]) -> Result<String, ElmaError> {
    let bytes: Vec<u8> = data.into_iter()
                             .take_while(|&&d| d != 0)
                             .cloned()
                             .collect();

    let trimmed = String::from_utf8(bytes)?;
    Ok(trimmed)
}

/// Converts the string-as-i32 times in top10 list to strings.
///
/// # Examples
/// Thanks to the genious data structure in Elma files, the best times in a level are represented
/// visually as a string, but stored as a i32. This function will convert the i32 time to a string
/// formatted as "00:00,00".
///
/// ```
/// let time: i32 = 2039;
/// let formatted = elma::time_format(time).unwrap();
/// assert_eq!("00:20,39", formatted);
/// ```
pub fn time_format (time: i32) -> Result<String, ElmaError> {
    let time = time.to_string().into_bytes();
    let mut formatted = String::from("00:00,00").into_bytes();

    // If input time is longer than 6 characters, return max time.
    if time.len() > 6 { return Ok(String::from("59:59,99")); }

    let mut n = 7;
    for byte in time.iter().rev() {
        // If first digit of minutes or seconds are over 5, return error.
        if ((n == 3) || (n == 0)) && (*byte > 53) { return Err(ElmaError::InvalidTimeFormat); }

        formatted[n] = *byte;
        if n == 6 || n == 3 {
            n -= 2;
        } else if n > 0 {
            n -= 1;
        }
    }

    let time = String::from_utf8(formatted)?;
    Ok(time)
}

/// Pads a string with null bytes.
///
/// # Examples
/// When converting strings to bytes for use in an Elma file, you need to pad it to a certain
/// length depending on the field. This function creates a new zero-filled vector to `pad` size,
/// then fills in the string.
///
/// ```
/// let string = String::from("Elma");
/// let padded = elma::string_null_pad(&string, 10).unwrap();
/// assert_eq!(&padded, &[0x45, 0x6C, 0x6D, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
/// ```
pub fn string_null_pad (name: &str, pad: usize) -> Result<Vec<u8>, ElmaError> {
    let name = name.as_bytes();

    // first check if string is ASCII
    if !name.is_ascii() { return Err(ElmaError::NonASCII) }
    // padding shorter than string
    if name.len() > pad { return Err(ElmaError::PaddingTooShort((pad as isize - name.len() as isize) as isize)) }

    let mut bytes = vec![0u8; pad];
    for (n, char) in name.iter().enumerate() {
        bytes[n] = *char;
    }
    Ok(bytes)
}

/// Diameter of player head.
pub const HEAD_DIAMETER: f64 = 0.476;
/// Radius of player head.
pub const HEAD_RADIUS: f64 = 0.238;
/// Diameter of objects (and wheels).
pub const OBJECT_DIAMETER: f64 = 0.8;
/// Radius of objects (and wheels).
pub const OBJECT_RADIUS: f64 = 0.4;
// Magic arbitrary number signifying end-of-data in level file.
const EOD: i32 = 0x0067103A;
// Magic arbitrary number signifying end-of-file in level file.
const EOF: i32 = 0x00845D52;
// Empty top10 data.
const EMPTY_TOP10: [u8; 688] = [0x15,0x05,0x6A,0xB7,0x89,0xED,0x59,0xC4,0x48,0xFF,0x8F,0x73,0x76,
                                0xBC,0x70,0xC0,0xDF,0x57,0xB4,0x2F,0x0D,0x9E,0x07,0xBC,0x63,0x08,
                                0x6F,0x8A,0x09,0x28,0xAD,0x38,0xE0,0x73,0xF9,0xA0,0x80,0x00,0x00,
                                0x8A,0x37,0x6F,0x69,0x60,0x17,0x09,0x77,0x41,0xAC,0x8C,0x26,0x34,
                                0xAC,0x02,0xA7,0x98,0xC9,0xA5,0xFE,0x80,0xDF,0xC0,0x97,0xB4,0x50,
                                0xD7,0x1D,0xCA,0x44,0x41,0xFB,0x47,0xF7,0xD8,0x60,0x38,0x84,0xCB,
                                0x4C,0x82,0x31,0x24,0x9A,0x91,0x76,0xC9,0x0E,0x6B,0xCC,0x75,0xFC,
                                0xCC,0xF8,0xA5,0x74,0x98,0x8E,0x02,0xBB,0x68,0x44,0xCB,0x4C,0xBD,
                                0x99,0xC4,0x34,0xF4,0x8B,0x7A,0x4C,0x68,0x58,0x53,0x9A,0x70,0xCD,
                                0x4F,0x2B,0xCC,0xD1,0x1A,0xE0,0x80,0x2E,0x8F,0x73,0x34,0xE7,0x0E,
                                0xD4,0x46,0x1D,0xBD,0x02,0x9A,0xBF,0x33,0x27,0x84,0x06,0x58,0x11,
                                0xA4,0x10,0xE4,0x1D,0x75,0x2A,0xB0,0x78,0x2F,0xE6,0x06,0x86,0xEF,
                                0xA4,0xD5,0xF9,0xFC,0x91,0x2E,0x54,0x74,0x5D,0xF8,0x91,0xB1,0xF6,
                                0xA2,0x55,0xF9,0x23,0xFB,0x47,0x8E,0x0F,0x45,0xF4,0xE7,0x5D,0x54,
                                0x39,0x8C,0xD1,0x2E,0x8F,0x38,0xBF,0x8F,0xBB,0x68,0xA0,0x38,0xD3,
                                0xC8,0x89,0x49,0x91,0x3B,0xBD,0x02,0x03,0x88,0xE5,0x2C,0x7F,0x9C,
                                0xB5,0x93,0x5F,0x57,0xA0,0xA1,0xE4,0x4B,0x11,0x5C,0xFD,0x6B,0x84,
                                0x34,0xC6,0xFC,0x91,0xB1,0x17,0x09,0x91,0x3B,0x96,0xA5,0x74,0x98,
                                0x3F,0xBD,0x5E,0x7D,0xA6,0x89,0x84,0x27,0xE0,0x52,0x64,0xA7,0xA5,
                                0xA2,0x90,0x40,0x4F,0x73,0x34,0x98,0x04,0x90,0x1F,0xCD,0xB8,0xA5,
                                0x81,0x43,0x88,0xC4,0xD8,0xC9,0xCC,0x19,0x2D,0x1E,0x97,0x65,0x46,
                                0xFC,0x35,0x58,0x32,0xD7,0x2A,0x2D,0xFD,0x4A,0x16,0x98,0x81,0xDA,
                                0x84,0x9D,0x19,0x5B,0x8C,0xB0,0x85,0x15,0x55,0xBE,0x38,0xA5,0xFE,
                                0x94,0x95,0xCB,0xE3,0xDA,0x56,0x22,0x69,0x04,0xB1,0x8D,0x07,0x46,
                                0x93,0x6C,0xA6,0x34,0x5D,0x54,0xD0,0xF8,0x84,0x90,0x1F,0xE1,0xC3,
                                0xB6,0x67,0xED,0x38,0x98,0x60,0xF6,0x60,0xC8,0x5B,0x02,0x5F,0x1C,
                                0xC2,0xA7,0x5D,0xA3,0x7E,0xCF,0x24,0xA7,0x1B,0x7F,0x7B,0xDE,0x42,
                                0xA7,0xD3,0xF6,0xAF,0xAB,0xCC,0xA3,0x64,0x03,0x2C,0x23,0x15,0xBE,
                                0xAE,0x40,0x4F,0x45,0xF4,0x50,0x05,0x7E,0xE9,0xF7,0xCB,0xFD,0x8C,
                                0xFF,0x8F,0x52,0xFB,0x47,0x53,0x1D,0x33,0x76,0xBC,0xAB,0xBF,0x40,
                                0xB8,0xE0,0x17,0xEF,0x00,0x0D,0x42,0x3E,0x66,0x13,0xF6,0x53,0xF6,
                                0x8E,0x1C,0xB5,0xB4,0x50,0xCA,0xBA,0x1E,0xB8,0x1B,0x02,0x10,0x4D,
                                0xE6,0x89,0x84,0xEC,0xE1,0xF1,0x03,0xDD,0xEB,0x9E,0xB8,0xBF,0xA9,
                                0x32,0xFE,0xCF,0x80,0x21,0xC3,0x2C,0x7F,0x26,0x4E,0xC0,0x48,0x68,
                                0xAD,0xE3,0x15,0xEC,0xBA,0x11,0x48,0x4E,0xD4,0x74,0x71,0x24,0x9A,
                                0x28,0xAD,0xB5,0xB4,0x98,0x04,0x1A,0x56,0xD3,0xE2,0xBE,0x94,0xD0,
                                0x12,0xFB,0xB0,0x57,0x65,0xBC,0xAB,0x56,0x77,0x06,0xE2,0x3B,0x5B,
                                0xF5,0x3E,0x45,0x01,0x15,0xE5,0xAF,0x69,0x04,0x6F,0x21,0xE4,0x31,
                                0xD5,0x34,0xAC,0x30,0x0F,0x24,0xD5,0x06,0xA7,0x3C,0x91,0x5C,0xB5,
                                0x2A,0x68,0xCE,0x6B,0x84,0x41,0x36,0x60,0xC8,0x13,0xB4,0xB9,0xFC,
                                0xAB,0x63,0x9F,0xB3,0x3B,0x19,0xC4,0xD8,0x11,0xA4,0x1D,0xA3,0x08,
                                0xB7,0x89,0xCC,0xEB,0x9E,0x00,0x00,0xF3,0xCB,0x1E,0xC5,0xE7,0x77,
                                0x7C,0x28,0xA0,0x80,0x48,0x89,0x49,0x1B,0xD4,0x88,0xC4,0x55,0x06,
                                0xCE,0xA6,0xAA,0xFF,0x8F,0xBB,0x2D,0x38,0x91,0xB1,0xBB,0xA3,0x8B,
                                0x59,0xB7,0xC4,0xEC,0x30,0x4A,0xE8,0xA0,0xDC,0x7A,0xE3,0xB9,0xA0,
                                0x38,0xED,0xC2,0xD5,0xBE,0x38,0x28,0x16,0x98,0x6D,0xCF,0x73,0x55,
                                0x13,0x4B,0x4C,0x19,0xF2,0xA9,0x04,0x9D,0xEB,0x42,0x8D,0x21,0xD7,
                                0x4B,0x2B,0x07,0x9B,0xDB,0x23,0x50,0x6E,0x82,0xD5,0xD8,0x04,0x34,
                                0x15,0xBE,0x38,0xED,0xB5,0x10,0xB6,0x53,0x79,0xF5,0x58,0x32,0x88,
                                0x20,0xE9,0xD6,0x98,0xEA,0xDE,0x97,0x2A,0x96,0x6A,0xA3,0x5D,0x0C,
                                0xF8,0x2F,0xC5,0xAC,0x6B,0x56,0xC6,0x79,0x51,0xD2,0x29,0x25,0x67,
                                0xED,0x59,0xFF,0x9C,0x38,0xD3,0xA7,0x84,0xCB,0x04,0x48,0x6F,0xC5,
                                0x22,0x97,0xC1,0x36,0xAF,0x14,0xC3,0x95,0xD8,0x60,0xE9,0x4C];
// Magic arbitrary number to signify end of replay file.
const EOR: i32 = 0x00492F75;
