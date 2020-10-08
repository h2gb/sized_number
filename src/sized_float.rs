use serde::{Serialize, Deserialize};
use byteorder::{ReadBytesExt, ByteOrder};
use simple_error::{SimpleResult, bail};
use std::fmt::*;

use crate::display_options::ScientificOptions;

pub type Context<'a> = std::io::Cursor<&'a Vec<u8>>;

#[derive(Debug, Clone, Copy)]
pub struct SizedFloat<T>
where
    T: LowerExp + UpperExp + Display + Copy
{
    value: T,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum FloatDisplay {
    Decimal,
    Scientific(ScientificOptions),
}

impl SizedFloat<f32> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_f32::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl SizedFloat<f64> {
    pub fn read<E: ByteOrder>(context: &Context) -> SimpleResult<Self> {
        Ok(Self { value: match context.clone().read_f64::<E>() {
            Ok(i) => i,
            Err(e) => bail!("Couldn't read: {}", e),
        }})
    }
}

impl<T> SizedFloat<T>
where
    T: LowerExp + UpperExp + Display + Copy
{
    pub fn to_string(&self, display: FloatDisplay) -> String {
        match display {
            FloatDisplay::Decimal => {
                format!("{}", self.value)
            },
            FloatDisplay::Scientific(options) => {
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
    fn test_decimal_f32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x41\xc8\x00\x00\x40\x48\xf5\xc3".to_vec();
        let context = Context::new(&data);

        type TestType = f32;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "NaN"),
            (   8,    "25"), // From https://en.wikipedia.org/wiki/Single-precision_floating-point_format#Converting_from_single-precision_binary_to_decimal
            (  12,    "3.14"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedFloat<TestType> = SizedFloat::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(FloatDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_f64_big_endian() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f\x40\x09\x33\x33\x33\x33\x33\x33".to_vec();
        let context = Context::new(&data);

        type TestType = f64;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "3.14"),
            (   8,    "3.15"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedFloat<TestType> = SizedFloat::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(FloatDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_f64_little_endian() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x1F\x85\xEB\x51\xB8\x1E\x09\x40\x33\x33\x33\x33\x33\x33\x09\x40".to_vec();
        let context = Context::new(&data);

        type TestType = f64;
        type TestEndian = LittleEndian;

        let tests = vec![
            // index  expected
            (   0,    "3.14"),
            (   8,    "3.15"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedFloat<TestType> = SizedFloat::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(FloatDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_exponent_f64() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f\x40\x09\x33\x33\x33\x33\x33\x33".to_vec();
        let context = Context::new(&data);

        type TestType = f64;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  uppercase expected
            (   0,    false,    "3.14e0"),
            (   8,    false,    "3.15e0"),
            (   0,    true,     "3.14E0"),
            (   8,    true,     "3.15E0"),
        ];

        for (index, uppercase, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedFloat<TestType> = SizedFloat::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(FloatDisplay::Scientific(ScientificOptions {
                uppercase: uppercase,
            })));
        }

        Ok(())
    }
}
