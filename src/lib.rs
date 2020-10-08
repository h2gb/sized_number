#![allow(dead_code)] // TODO: Disable this

use serde::{Serialize, Deserialize};

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;
pub mod sized_integer;
pub mod sized_float;

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
