//! [![Crate](https://img.shields.io/crates/v/sized_number.svg)](https://crates.io/crates/sized_number)
//!
//! A simple library for reading differently-sized integers and floats.
//!
//! While writing h2gb, I needed a way to dynamically read integers from a
//! Vec of u8 bytes. Libs like `byteorder` and `io::Cursor` nearly has the
//! right functionality, but weren't quite flexible enough.
//!
//! This library wraps / uses those modules to simplify reading arbitrary values
//! from a cursor, and storing / displaying them with user-controlled settings.
//!
//! # Example
//!
//! TODO

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use simple_error::{SimpleResult, bail};
use std::fmt::{LowerHex, LowerExp, Octal, Binary, Display}; // TODO
use std::io;
use std::mem;

#[cfg(feature = "serialize")]
use serde::{Serialize, Deserialize};

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

pub fn new_context(v: &Vec<u8>, offset: u64) -> Context {
    let mut c = Context::new(v);
    c.set_position(offset);

    c
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ScientificOptions {
    pub uppercase: bool,
}

impl Default for ScientificOptions {
    fn default() -> Self {
        Self {
            uppercase: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HexOptions {
    pub uppercase: bool,
    pub prefix: bool,
    pub padded: bool,
}

impl Default for HexOptions {
    fn default() -> Self {
        Self {
            uppercase: false,
            prefix: true,
            padded: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct BinaryOptions {
    pub padded: bool,
}

impl Default for BinaryOptions {
    fn default() -> Self {
        Self {
            padded: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum SizedDisplay {
    Hex(HexOptions),
    Decimal,
    Octal,
    Binary(BinaryOptions),
    Scientific(ScientificOptions),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum SizedDefinition {
    U8,
    U16(Endian),
    U32(Endian),
    U64(Endian),
    U128(Endian),

    I8,
    I16(Endian),
    I32(Endian),
    I64(Endian),
    I128(Endian),

    F32(Endian),
    F64(Endian),
}

fn display_hex(v: Box<dyn LowerHex>, options: HexOptions) -> String {
    let v = v.as_ref();

    let mut h = match options.padded {
        // No padding is easy
        false => format!("{:x}",   v),

        // Padding requires a bit more tinkering to do dynamically
        true => {
            match (options.padded, mem::size_of_val(v) * 2) {
                (true, 2)   => format!(  "{:02x}",  v),
                (true, 4)   => format!(  "{:04x}",  v),
                (true, 8)   => format!(  "{:08x}",  v),
                (true, 16)  => format!(  "{:016x}", v),
                (true, 32)  => format!(  "{:032x}", v),

                // When not padded, or in doubt about length, just print normally
                (_, _)      => format!(  "{:x}",     v),
            }
        }
    };

    // There's no way to make the parameter both LowerHex and UpperHex
    if options.uppercase {
        h = h.to_uppercase();
    }

    if options.prefix {
        h = format!("0x{}", h);
    }

    h
}

fn display_decimal(v: Box<dyn Display>) -> String {
    let v = v.as_ref();

    format!("{}", v)
}

fn display_octal(v: Box<dyn Octal>) -> String {
    let v = v.as_ref();

    format!("{:o}", v)
}

fn display_binary(v: Box<dyn Binary>, options: BinaryOptions) -> String {
    let v = v.as_ref();

    match options.padded {
        false => format!("{:b}", v),
        true => {
            match mem::size_of_val(v) * 8 {
                8   => format!("{:08b}",   v),
                16  => format!("{:016b}",  v),
                32  => format!("{:032b}",  v),
                64  => format!("{:064b}",  v),
                128 => format!("{:0128b}", v),
                _   => format!("{:b}",     v),
            }
        }
    }
}

fn display_scientific(v: Box<dyn LowerExp>, options: ScientificOptions) -> String {
    let mut v = format!("{:e}", v.as_ref());

    if options.uppercase {
        v = v.to_uppercase();
    }

    v
}

impl SizedDefinition {
    pub fn size(self) -> u64 {
        match self {
            Self::U8      => 1,
            Self::U16(_)  => 2,
            Self::U32(_)  => 4,
            Self::U64(_)  => 8,
            Self::U128(_) => 16,

            Self::I8      => 1,
            Self::I16(_)  => 2,
            Self::I32(_)  => 4,
            Self::I64(_)  => 8,
            Self::I128(_) => 16,

            Self::F32(_)  => 4,
            Self::F64(_)  => 8,
        }
    }

    fn to_string_internal(self, context: &Context, display: SizedDisplay) -> io::Result<String> {
        match self {
            Self::U8 => {
                let v = Box::new(context.clone().read_u8()?);
                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::U16(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_u16::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_u16::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::U32(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_u32::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_u32::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::U64(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_u64::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_u64::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::U128(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_u128::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_u128::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::I8 => {
                let v = Box::new(context.clone().read_i8()?);

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::I16(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_i16::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_i16::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::I32(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_i32::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_i32::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::I64(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_i64::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_i64::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::I128(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_i128::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_i128::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(options)        => Ok(display_hex(v, options)),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Ok(display_octal(v)),
                    SizedDisplay::Binary(options)     => Ok(display_binary(v, options)),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::F32(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_f32::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_f32::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(_)              => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as hex")),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as octal")),
                    SizedDisplay::Binary(_)           => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as binary")),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },

            Self::F64(endian) => {
                let v = match endian {
                    Endian::Big => Box::new(context.clone().read_f64::<BigEndian>()?),
                    Endian::Little => Box::new(context.clone().read_f64::<LittleEndian>()?),
                };

                match display {
                    SizedDisplay::Hex(_)              => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as hex")),
                    SizedDisplay::Decimal             => Ok(display_decimal(v)),
                    SizedDisplay::Octal               => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as octal")),
                    SizedDisplay::Binary(_)           => Err(io::Error::new(io::ErrorKind::Other, "Floats can't be displayed as binary")),
                    SizedDisplay::Scientific(options) => Ok(display_scientific(v, options)),
                }
            },
        }
    }

    pub fn to_string(self, context: &Context, display: SizedDisplay) -> SimpleResult<String> {
        match self.to_string_internal(context, display) {
            Ok(s) => Ok(s),
            Err(e) => bail!("Couldn't convert to string: {}", e),
        }
    }

    pub fn to_u64(self, context: &Context) -> SimpleResult<u64> {
        match self {
            Self::U8 => {
                match context.clone().read_u8() {
                    Ok(v)  => Ok(v as u64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::U16(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_u16::<BigEndian>(),
                    Endian::Little => context.clone().read_u16::<LittleEndian>(),
                };

                match v {
                    Ok(v)  => Ok(v as u64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::U32(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_u32::<BigEndian>(),
                    Endian::Little => context.clone().read_u32::<LittleEndian>(),
                };

                match v {
                    Ok(v)  => Ok(v as u64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::U64(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_u64::<BigEndian>(),
                    Endian::Little => context.clone().read_u64::<LittleEndian>(),
                };

                match v {
                    Ok(v)  => Ok(v as u64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },

            // None of these can become u32
            Self::U128(_) => bail!("Can't convert u128 into u64"),

            Self::I8      => bail!("Can't convert i8 (signed) into u64"),
            Self::I16(_)  => bail!("Can't convert i16 (signed) into u64"),
            Self::I32(_)  => bail!("Can't convert i32 (signed) into u64"),
            Self::I64(_)  => bail!("Can't convert i64 (signed) into u64"),
            Self::I128(_) => bail!("Can't convert i128 (signed) into u64"),

            Self::F32(_)  => bail!("Can't convert floating point into u64"),
            Self::F64(_)  => bail!("Can't convert floating point into u64"),
        }
    }

    pub fn to_i64(self, context: &Context) -> SimpleResult<i64> {
        match self {
            // Don't let unsigned values become signed
            Self::U8      => bail!("Can't convert i8 (signed) into i64"),
            Self::U16(_)  => bail!("Can't convert i16 (signed) into i64"),
            Self::U32(_)  => bail!("Can't convert i32 (signed) into i64"),
            Self::U64(_)  => bail!("Can't convert i64 (signed) into i64"),
            Self::U128(_) => bail!("Can't convert i128 (signed) into i64"),

            Self::I8 => {
                match context.clone().read_i8() {
                    Ok(v) => Ok(v as i64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::I16(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_i16::<BigEndian>(),
                    Endian::Little => context.clone().read_i16::<LittleEndian>(),
                };

                match v {
                    Ok(v) => Ok(v as i64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::I32(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_i32::<BigEndian>(),
                    Endian::Little => context.clone().read_i32::<LittleEndian>(),
                };

                match v {
                    Ok(v) => Ok(v as i64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },
            Self::I64(endian) => {
                let v = match endian {
                    Endian::Big => context.clone().read_i64::<BigEndian>(),
                    Endian::Little => context.clone().read_i64::<LittleEndian>(),
                };

                match v {
                    Ok(v) => Ok(v as i64),
                    Err(e) => bail!("Failed to read data: {}", e),
                }
            },


            // 128 bit can't go into 64 bit
            Self::I128(_) => bail!("Can't convert u128 into i64"),

            // Float certainly can't
            Self::F32(_)  => bail!("Can't convert floating point into i64"),
            Self::F64(_)  => bail!("Can't convert floating point into i64"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;

    #[test]
    fn test_hex_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "0"),
            (   0,    true,       false,   false,    "0"),
            (   0,    false,      true,    false,    "0x0"),
            (   0,    false,      false,   true,     "00"),
            (   0,    true,       true,    true,     "0x00"),

            // index  uppercase   prefix   padded    expected
            (   1,    false,      false,   false,    "7f"),
            (   1,    true,       false,   false,    "7F"),
            (   1,    false,      true,    false,    "0x7f"),
            (   1,    false,      false,   true,     "7f"),
            (   1,    true,       true,    true,     "0x7F"),

            // index  uppercase   prefix   padded    expected
            (   2,    false,      false,   false,    "80"),
            (   2,    true,       false,   false,    "80"),
            (   2,    false,      true,    false,    "0x80"),
            (   2,    false,      false,   true,     "80"),
            (   2,    true,       true,    true,     "0x80"),

            // index  uppercase   prefix   padded    expected
            (   3,    false,      false,   false,    "ff"),
            (   3,    true,       false,   false,    "FF"),
            (   3,    false,      true,    false,    "0xff"),
            (   3,    false,      false,   true,     "ff"),
            (   3,    true,       true,    true,     "0xFF"),

        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U8.to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_hex_u16() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "0"),
            (   0,    true,       false,   false,    "0"),
            (   0,    false,      true,    false,    "0x0"),
            (   0,    false,      false,   true,     "0000"),
            (   0,    true,       true,    true,     "0x0000"),

            // index  uppercase   prefix   padded    expected
            (   2,    false,      false,   false,    "1234"),
            (   2,    true,       false,   false,    "1234"),
            (   2,    false,      true,    false,    "0x1234"),
            (   2,    false,      false,   true,     "1234"),
            (   2,    true,       true,    true,     "0x1234"),

            // index  uppercase   prefix   padded    expected
            (   4,    false,      false,   false,    "ffff"),
            (   4,    true,       false,   false,    "FFFF"),
            (   4,    false,      true,    false,    "0xffff"),
            (   4,    false,      false,   true,     "ffff"),
            (   4,    true,       true,    true,     "0xFFFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U16(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_hex_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "1234"),
            (   0,    true,       false,   false,    "1234"),
            (   0,    false,      true,    false,    "0x1234"),
            (   0,    false,      false,   true,     "00001234"),
            (   0,    true,       true,    true,     "0x00001234"),

            // index  uppercase   prefix   padded    expected
            (   4,    false,      false,   false,    "ffffffff"),
            (   4,    true,       false,   false,    "FFFFFFFF"),
            (   4,    false,      true,    false,    "0xffffffff"),
            (   4,    false,      false,   true,     "ffffffff"),
            (   4,    true,       true,    true,     "0xFFFFFFFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_hex_u64_big_endian() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "1234ffffffff"),
            (   0,    true,       false,   false,    "1234FFFFFFFF"),
            (   0,    false,      true,    false,    "0x1234ffffffff"),
            (   0,    false,      false,   true,     "00001234ffffffff"),
            (   0,    true,       true,    true,     "0x00001234FFFFFFFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U64(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_hex_u64_little_endian() -> SimpleResult<()> {
        let data = b"\x00\x12\x34\xFF\xFF\xFF\xFF\x00".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "ffffffff341200"),
            (   0,    true,       false,   false,    "FFFFFFFF341200"),
            (   0,    false,      true,    false,    "0xffffffff341200"),
            (   0,    false,      false,   true,     "00ffffffff341200"),
            (   0,    true,       true,    true,     "0x00FFFFFFFF341200"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U64(Endian::Little).to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_hex_u128_big_endian() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa\xbb\xcc\xdd\xee\xff".to_vec();

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "1"),
            (   0,    true,       false,   false,    "1"),
            (   0,    false,      true,    false,    "0x1"),
            (   0,    false,      false,   true,     "00000000000000000000000000000001"),
            (   0,    true,       true,    true,     "0x00000000000000000000000000000001"),

            // index  uppercase   prefix   padded    expected
            (   16,    false,      false,   false,    "112233445566778899aabbccddeeff"),
            (   16,    true,       false,   false,    "112233445566778899AABBCCDDEEFF"),
            (   16,    false,      true,    false,    "0x112233445566778899aabbccddeeff"),
            (   16,    false,      false,   true,     "00112233445566778899aabbccddeeff"),
            (   16,    true,       true,    true,     "0x00112233445566778899AABBCCDDEEFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U128(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Hex(HexOptions {
                        uppercase: uppercase,
                        prefix: prefix,
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "127"),
            (   2,    "128"),
            (   3,    "255"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U8.to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "127"),
            (   2,    "-128"),
            (   3,    "-1"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::I8.to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u16() -> SimpleResult<()> {
        let data = b"\x00\xFF\x00\x01\x00\x00\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "255"),
            (   2,    "1"),
            (   4,    "0"),
            (   6,    "65535"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U16(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x7f\xff\xff\xff\x80\x00\x00\x00".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "4294967295"),
            (   8,    "2147483647"),
            (  12,    "2147483648"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x7f\xff\xff\xff\x80\x00\x00\x00".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "-1"),
            (   8,    "2147483647"),
            (  12,    "-2147483648"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::I32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i64() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x7f\xff\xff\xff\xff\xff\xff\xff\x80\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff".to_vec();

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  8,    "9223372036854775807"),
            (  16,   "-9223372036854775808"),
            (  24,   "-1"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::I64(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u128() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  16,   "340282366920938463463374607431768211455"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U128(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i128() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  16,   "-1"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::I128(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "177"),
            (   2,    "200"),
            (   3,    "377"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U8.to_string(
                    &context,
                    SizedDisplay::Octal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u16() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   2,    "11064"),
            (   4,    "177777"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U16(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Octal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "11064"),
            (   2,    "2215177777"),
            (   4,    "37777777777"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Octal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u64() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "443237777777777"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U64(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Octal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_binary_i8() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\xab\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index   padded   expected
            (   0,     true,    "00000000"),
            (   1,     true,    "00000000"),
            (   2,     true,    "00010010"),
            (   3,     true,    "10101011"),
            (   4,     true,    "11111111"),
            (   5,     true,    "11111111"),

            (   0,     false,   "0"),
            (   1,     false,   "0"),
            (   2,     false,   "10010"),
            (   3,     false,   "10101011"),
            (   4,     false,   "11111111"),
            (   5,     false,   "11111111"),
        ];

        for (index, padded, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U8.to_string(
                    &context,
                    SizedDisplay::Binary(BinaryOptions {
                        padded: padded,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_scientific_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();

        let tests = vec![
            // index  uppercase  expected
            (   0,    false,     "0e0"),
            (   4,    false,     "2.147483647e9"),
            (   8,    false,     "2.147483648e9"),
            (  12,    false,     "4.294967295e9"),
            (   0,    true,      "0E0"),
            (   4,    true,      "2.147483647E9"),
            (   8,    true,      "2.147483648E9"),
            (  12,    true,      "4.294967295E9"),
        ];

        for (index, uppercase, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::U32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Scientific(ScientificOptions {
                        uppercase: uppercase,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_scientific_i32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();

        let tests = vec![
            // index  uppercase  expected
            (   0,    false,     "0e0"),
            (   4,    false,     "2.147483647e9"),
            (   8,    false,     "-2.147483648e9"),
            (  12,    false,     "-1e0"),
            (   0,    true,      "0E0"),
            (   4,    true,      "2.147483647E9"),
            (   8,    true,      "-2.147483648E9"),
            (  12,    true,      "-1E0"),
        ];

        for (index, uppercase, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::I32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Scientific(ScientificOptions {
                        uppercase: uppercase,
                    })
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_f32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x41\xc8\x00\x00\x40\x48\xf5\xc3".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "NaN"),
            (   8,    "25"), // From https://en.wikipedia.org/wiki/Single-precision_floating-point_format#Converting_from_single-precision_binary_to_decimal
            (  12,    "3.14"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::F32(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_f64_big_endian() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f\x40\x09\x33\x33\x33\x33\x33\x33".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "3.14"),
            (   8,    "3.15"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::F64(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_decimal_f64_little_endian() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x1F\x85\xEB\x51\xB8\x1E\x09\x40\x33\x33\x33\x33\x33\x33\x09\x40".to_vec();

        let tests = vec![
            // index  expected
            (   0,    "3.14"),
            (   8,    "3.15"),
        ];

        for (index, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::F64(Endian::Little).to_string(
                    &context,
                    SizedDisplay::Decimal
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_exponent_f64() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f\x40\x09\x33\x33\x33\x33\x33\x33".to_vec();

        let tests = vec![
            // index  uppercase expected
            (   0,    false,    "3.14e0"),
            (   8,    false,    "3.15e0"),
            (   0,    true,     "3.14E0"),
            (   8,    true,     "3.15E0"),
        ];

        for (index, uppercase, expected) in tests {
            let context = new_context(&data, index);

            assert_eq!(
                expected,
                SizedDefinition::F64(Endian::Big).to_string(
                    &context,
                    SizedDisplay::Scientific(ScientificOptions {
                        uppercase: uppercase,
                    }),
                )?
            );
        }

        Ok(())
    }

    #[test]
    fn test_buffer_too_short() -> SimpleResult<()> {
        let data = b"".to_vec();
        assert!(SizedDefinition::I8.to_string(&Context::new(&data), SizedDisplay::Decimal).is_err());

        let data = b"A".to_vec();
        assert!(SizedDefinition::I16(Endian::Big).to_string(&Context::new(&data), SizedDisplay::Decimal).is_err());

        let data = b"AAA".to_vec();
        assert!(SizedDefinition::I32(Endian::Big).to_string(&Context::new(&data), SizedDisplay::Decimal).is_err());

        let data = b"AAAAAAA".to_vec();
        assert!(SizedDefinition::I64(Endian::Big).to_string(&Context::new(&data), SizedDisplay::Decimal).is_err());

        let data = b"AAAAAAAAAAAAAAA".to_vec();
        assert!(SizedDefinition::I128(Endian::Big).to_string(&Context::new(&data), SizedDisplay::Decimal).is_err());

        Ok(())
    }
}
