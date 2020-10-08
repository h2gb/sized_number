// TODO: Only import if feature is enabled
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ScientificOptions {
    pub uppercase: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HexOptions {
    pub uppercase: bool,
    pub prefix: bool,
    pub padded: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct BinaryOptions {
    pub padded: bool,
}

