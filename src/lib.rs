//! [![Crate](https://img.shields.io/crates/v/sized_integer.svg)](https://crates.io/crates/sized_integer)
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
#![allow(dead_code)] // TODO: Disable this

use byteorder::{BigEndian, LittleEndian};
use simple_error::SimpleResult;

#[cfg(feature = "serialize")]
use serde::{Serialize, Deserialize};

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

pub mod display_options;

pub mod real_sized_integer;
pub mod sized_integer;
pub use sized_integer::{SizedInteger, SizedIntegerDisplay};

pub mod sized_float;
pub use sized_float::{SizedFloat, SizedFloatDisplay};

pub enum Endian {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum SizedNumber {
    EightBitUnsigned(SizedInteger<u8>, SizedIntegerDisplay),
    SixteenBitUnsigned(SizedInteger<u16>, SizedIntegerDisplay),
    ThirtyTwoBitUnsigned(SizedInteger<u32>, SizedIntegerDisplay),
    SixtyFourBitUnsigned(SizedInteger<u64>, SizedIntegerDisplay),
    OneTwentyEightBitUnsigned(SizedInteger<u128>, SizedIntegerDisplay),

    EightBitSigned(SizedInteger<i8>, SizedIntegerDisplay),
    SixteenBitSigned(SizedInteger<i16>, SizedIntegerDisplay),
    ThirtyTwoBitSigned(SizedInteger<i32>, SizedIntegerDisplay),
    SixtyFourBitSigned(SizedInteger<i64>, SizedIntegerDisplay),
    OneTwentyEightBitSigned(SizedInteger<i128>, SizedIntegerDisplay),

    ThirtyTwoBitFloat(SizedFloat<f32>, SizedFloatDisplay),
    SixtyFourBitFloat(SizedFloat<f64>, SizedFloatDisplay),
}

impl SizedNumber {
    pub fn read_u8(context: &Context, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(Self::EightBitUnsigned(SizedInteger::<u8>::read(context)?, display))
    }

    pub fn read_u16(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::SixteenBitUnsigned(SizedInteger::<u16>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::SixteenBitUnsigned(SizedInteger::<u16>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_u32(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::ThirtyTwoBitUnsigned(SizedInteger::<u32>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::ThirtyTwoBitUnsigned(SizedInteger::<u32>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_u64(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::SixtyFourBitUnsigned(SizedInteger::<u64>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::SixtyFourBitUnsigned(SizedInteger::<u64>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_u128(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::OneTwentyEightBitUnsigned(SizedInteger::<u128>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::OneTwentyEightBitUnsigned(SizedInteger::<u128>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_i8(context: &Context, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(Self::EightBitSigned(SizedInteger::<i8>::read(context)?, display))
    }

    pub fn read_i16(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::SixteenBitSigned(SizedInteger::<i16>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::SixteenBitSigned(SizedInteger::<i16>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_i32(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::ThirtyTwoBitSigned(SizedInteger::<i32>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::ThirtyTwoBitSigned(SizedInteger::<i32>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_i64(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::SixtyFourBitSigned(SizedInteger::<i64>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::SixtyFourBitSigned(SizedInteger::<i64>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_i128(context: &Context, endian: Endian, display: SizedIntegerDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::OneTwentyEightBitSigned(SizedInteger::<i128>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::OneTwentyEightBitSigned(SizedInteger::<i128>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_f32(context: &Context, endian: Endian, display: SizedFloatDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::ThirtyTwoBitFloat(SizedFloat::<f32>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::ThirtyTwoBitFloat(SizedFloat::<f32>::read::<LittleEndian>(context)?, display),
        })
    }

    pub fn read_f64(context: &Context, endian: Endian, display: SizedFloatDisplay) -> SimpleResult<Self> {
        Ok(match endian {
            Endian::BigEndian    => Self::SixtyFourBitFloat(SizedFloat::<f64>::read::<BigEndian>(context)?, display),
            Endian::LittleEndian => Self::SixtyFourBitFloat(SizedFloat::<f64>::read::<LittleEndian>(context)?, display),
        })
    }
}
