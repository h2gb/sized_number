#![allow(dead_code)] // TODO: Disable this

use serde::{Serialize, Deserialize};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use simple_error::{SimpleResult, bail};
use std::io::{self, Read};

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

mod integer;

// pub mod number_size;
// pub use number_size::NumberSize;

// pub mod number_display;
// pub use number_display::NumberDisplay;

// type SizedFormat = (NumberSize, Endian);

// pub const U8:         SizedFormat = (NumberSize::Eight,     Endian::BigEndian);
// pub const U16_BIG:    SizedFormat = (NumberSize::Sixteen,   Endian::BigEndian);
// pub const U16_LITTLE: SizedFormat = (NumberSize::Sixteen,   Endian::LittleEndian);
// pub const U32_BIG:    SizedFormat = (NumberSize::ThirtyTwo, Endian::BigEndian);
// pub const U32_LITTLE: SizedFormat = (NumberSize::ThirtyTwo, Endian::LittleEndian);
// pub const U64_BIG:    SizedFormat = (NumberSize::SixtyFour, Endian::BigEndian);
// pub const U64_LITTLE: SizedFormat = (NumberSize::SixtyFour, Endian::LittleEndian);

// pub fn number_size(format: SizedFormat) -> u64 {
//     format.0.size()
// }

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum NumberSize {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl NumberSize {
    pub fn size(self) -> u64 {
        match self {
            Self::Eight     => 1,
            Self::Sixteen   => 2,
            Self::ThirtyTwo => 4,
            Self::SixtyFour => 8,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use simple_error::SimpleResult;
//     use pretty_assertions::assert_eq;

//     #[test]
//     fn test_() -> SimpleResult<()> {
//         let data = b"\x00\x01\x02\x03\xFF\xFF\xFF\xFFBBBB".to_vec();
//         let mut c = Context::new(&data);

//         let number = SizedNumber::read(&c, SizedInputFormat {
//             size: NumberSize::Eight,
//             endian: Endian::BigEndian,
//         })?;
//         println!("Should be 0: {}", number.to_u64()?);

//         let number = SizedNumber::read(&c, SizedInputFormat {
//             size: NumberSize::ThirtyTwo,
//             endian: Endian::BigEndian,
//         })?;
//         println!("Should be 10203: {:x}", number.to_u64()?);

//         let number = SizedNumber::read(&c, SizedInputFormat {
//             size: NumberSize::ThirtyTwo,
//             endian: Endian::BigEndian,
//         })?;
//         println!("Should be 66051: {}", number.to_string(SizedOutputFormat {
//             signed: false,
//             format: NumberDisplay::Decimal,
//         })?);

//         c.set_position(4);
//         let number = SizedNumber::read(&c, SizedInputFormat {
//             size: NumberSize::ThirtyTwo,
//             endian: Endian::BigEndian,
//         })?;
//         println!("Should be 4294967295: {}", number.to_string(SizedOutputFormat {
//             signed: false,
//             format: NumberDisplay::Decimal,
//         })?);

//         let number = SizedNumber::read(&c, SizedInputFormat {
//             size: NumberSize::ThirtyTwo,
//             endian: Endian::BigEndian,
//         })?;
//         println!("Should be -1: {}", number.to_string(SizedOutputFormat {
//             signed: true,
//             format: NumberDisplay::Decimal,
//         })?);

//         Ok(())
//     }
// }
