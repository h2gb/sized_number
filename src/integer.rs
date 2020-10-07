use serde::{Serialize, Deserialize};
use byteorder::{ReadBytesExt, ByteOrder};
use simple_error::{SimpleResult, bail};
use std::mem;
use std::fmt::*;

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

pub struct SizedInteger<T>
where
    T: UpperHex + LowerHex + Octal + Binary + LowerExp + UpperExp + Display
{
    value: T,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HexOptions {
    uppercase: bool,
    prefix: bool,
    padded: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ScientificOptions {
    uppercase: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum NumberDisplay {
    Hex(HexOptions),
    Decimal,
    Octal,
    Binary,
    Scientific(ScientificOptions),
}

impl SizedInteger<u8> {
    pub fn read(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_u8() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<u16> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_u16::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<u32> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_u32::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<u64> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_u64::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<u128> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_u128::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<i8> {
    pub fn read(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_i8() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<i16> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_i16::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<i32> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_i32::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<i64> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_i64::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedInteger<i128> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_i128::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl<T> SizedInteger<T>
where
    T: UpperHex + LowerHex + Octal + Binary + LowerExp + UpperExp + Display
{
    pub fn to_string(&self, display: NumberDisplay) -> String {
        match display {
            NumberDisplay::Binary => {
                format!("{:b}", self.value)
            },
            NumberDisplay::Decimal => {
                format!("{}", self.value)
            },
            NumberDisplay::Hex(options) => {
                // Assume no padding
                let mut padding = "".to_string();

                // If the user wants padding, sort it out
                if options.padded {
                    // Do a lazy conversion to get the length
                    let current_len = format!("{:x}", self.value).len();

                    // Get the actual desired length for the type
                    let total_needed_len = mem::size_of::<T>() * 2;

                    // Create the padding string
                    if current_len < total_needed_len {
                        padding = str::repeat("0", total_needed_len - current_len);
                    }
                }

                // Do the rest of the formatting
                match (options.prefix, options.uppercase) {
                    (false, false) => format!("{}{:x}",   padding, self.value),
                    (false, true)  => format!("{}{:X}",   padding, self.value),
                    (true,  false) => format!("0x{}{:x}", padding, self.value),
                    (true,  true)  => format!("0x{}{:X}", padding, self.value),
                }
            },
            NumberDisplay::Octal => {
                format!("{:o}", self.value)
            },
            NumberDisplay::Scientific(options) => {
                match options.uppercase {
                    false => format!("{:e}", self.value),
                    true =>  format!("{:E}", self.value),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use byteorder::{BigEndian, LittleEndian};
    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;

    #[test]
    fn test_hex_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\xab\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "12ab"),
            (   0,    true,       false,   false,    "12AB"),
            (   0,    false,      true,    false,    "0x12ab"),
            (   0,    false,      false,   true,     "000012ab"),
            (   0,    true,       true,    true,     "0x000012AB"),

            (   4,    false,      false,    false,    "ffffffff"),
            (   4,    true,       false,    false,    "FFFFFFFF"),
            (   4,    false,      true,     false,    "0xffffffff"),
            (   4,    false,      false,    true,     "ffffffff"),
            (   4,    true,       true,     true,     "0xFFFFFFFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<u32> = SizedInteger::<u32>::read::<BigEndian>(&context)?;
            assert_eq!(expected, t.to_string(NumberDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }
}
