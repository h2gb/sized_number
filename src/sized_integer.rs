use serde::{Serialize, Deserialize};
use byteorder::{ReadBytesExt, ByteOrder};
use simple_error::{SimpleResult, bail};
use std::mem;
use std::fmt::*;

use crate::Context;
use crate::display_options::{ScientificOptions, HexOptions, BinaryOptions};

pub struct SizedInteger<T>
where
    T: UpperHex + LowerHex + Octal + Binary + LowerExp + UpperExp + Display
{
    value: T,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum IntegerDisplay {
    Hex(HexOptions),
    Decimal,
    Octal,
    Binary(BinaryOptions),
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
    pub fn size() -> usize {
        mem::size_of::<T>()
    }

    pub fn to_string(&self, display: IntegerDisplay) -> String {
        match display {
            IntegerDisplay::Binary(options) => {
                match options.padded {
                    false => format!("{:b}", self.value),
                    true => {
                        match Self::size() * 8 {
                            8   => format!("{:08b}", self.value),
                            16  => format!("{:016b}", self.value),
                            32  => format!("{:032b}", self.value),
                            64  => format!("{:064b}", self.value),
                            128 => format!("{:0128b}", self.value),
                            _   => format!("{:b}", self.value),
                        }
                    }
                }
            },
            IntegerDisplay::Decimal => {
                format!("{}", self.value)
            },
            IntegerDisplay::Hex(options) => {
                match options.padded {
                    // No padding is easy
                    false => {
                        match (options.prefix, options.uppercase) {
                            (false, false) => format!("{:x}",   self.value),
                            (false, true)  => format!("{:X}",   self.value),
                            (true,  false) => format!("0x{:x}", self.value),
                            (true,  true)  => format!("0x{:X}", self.value),
                        }
                    }

                    // Padding requires a bit more tinkering to do dynamically
                    true => {
                        match (Self::size() * 2, options.uppercase, options.prefix) {
                            (2, false, false)  => format!(  "{:02x}", self.value),
                            (2, false, true)   => format!("0x{:02x}", self.value),
                            (2, true, false)   => format!("{:02X}", self.value),
                            (2, true, true)    => format!("0x{:02X}", self.value),

                            (4, false, false)  => format!(  "{:04x}", self.value),
                            (4, false, true)   => format!("0x{:04x}", self.value),
                            (4, true, false)   => format!("{:04X}", self.value),
                            (4, true, true)    => format!("0x{:04X}", self.value),

                            (8, false, false)  => format!(  "{:08x}", self.value),
                            (8, false, true)   => format!("0x{:08x}", self.value),
                            (8, true, false)   => format!("{:08X}", self.value),
                            (8, true, true)    => format!("0x{:08X}", self.value),

                            (16, false, false) => format!(  "{:016x}", self.value),
                            (16, false, true)  => format!("0x{:016x}", self.value),
                            (16, true, false)  => format!("{:016X}", self.value),
                            (16, true, true)   => format!("0x{:016X}", self.value),

                            (32, false, false) => format!(  "{:032x}", self.value),
                            (32, false, true)  => format!("0x{:032x}", self.value),
                            (32, true, false)  => format!("{:032X}", self.value),
                            (32, true, true)   => format!("0x{:032X}", self.value),

                            // When in doubt, just print normally
                            (_, false, false) => format!(  "{:x}", self.value),
                            (_, false, true)  => format!("0x{:x}", self.value),
                            (_, true, false)  => format!("{:X}", self.value),
                            (_, true, true)   => format!("0x{:X}", self.value),
                        }
                    }
                }
            },
            IntegerDisplay::Octal => {
                format!("{:o}", self.value)
            },
            IntegerDisplay::Scientific(options) => {
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
    fn test_hex_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u8;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_hex_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u32;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_hex_u16() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u16;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_hex_u64_big_endian() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u64;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "1234ffffffff"),
            (   0,    true,       false,   false,    "1234FFFFFFFF"),
            (   0,    false,      true,    false,    "0x1234ffffffff"),
            (   0,    false,      false,   true,     "00001234ffffffff"),
            (   0,    true,       true,    true,     "0x00001234FFFFFFFF"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_hex_u64_little_endian() -> SimpleResult<()> {
        let data = b"\x00\x12\x34\xFF\xFF\xFF\xFF\x00".to_vec();
        let context = Context::new(&data);

        type TestType = u64;
        type TestEndian = LittleEndian;

        let tests = vec![
            // index  uppercase   prefix   padded    expected
            (   0,    false,      false,   false,    "ffffffff341200"),
            (   0,    true,       false,   false,    "FFFFFFFF341200"),
            (   0,    false,      true,    false,    "0xffffffff341200"),
            (   0,    false,      false,   true,     "00ffffffff341200"),
            (   0,    true,       true,    true,     "0x00FFFFFFFF341200"),
        ];

        for (index, uppercase, prefix, padded, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_hex_u128_big_endian() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa\xbb\xcc\xdd\xee\xff".to_vec();
        let context = Context::new(&data);

        type TestType = u128;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Hex(HexOptions {
                uppercase: uppercase,
                prefix: prefix,
                padded: padded,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u8;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "127"),
            (   2,    "128"),
            (   3,    "255"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = i8;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "127"),
            (   2,    "-128"),
            (   3,    "-1"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u16() -> SimpleResult<()> {
        let data = b"\x00\xFF\x00\x01\x00\x00\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u16;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "255"),
            (   2,    "1"),
            (   4,    "0"),
            (   6,    "65535"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x7f\xff\xff\xff\x80\x00\x00\x00".to_vec();
        let context = Context::new(&data);

        type TestType = u32;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "4294967295"),
            (   8,    "2147483647"),
            (  12,    "2147483648"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\xff\xff\xff\xff\x7f\xff\xff\xff\x80\x00\x00\x00".to_vec();
        let context = Context::new(&data);

        type TestType = i32;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   4,    "-1"),
            (   8,    "2147483647"),
            (  12,    "-2147483648"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u64() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x7f\xff\xff\xff\xff\xff\xff\xff\x80\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff".to_vec();
        let context = Context::new(&data);

        type TestType = i64;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  8,    "9223372036854775807"),
            (  16,   "-9223372036854775808"),
            (  24,   "-1"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_u128() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u128;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  16,   "340282366920938463463374607431768211455"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_decimal_i128() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = i128;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (  0,    "0"),
            (  16,   "-1"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Decimal));
        }

        Ok(())
    }

    #[test]
    fn test_octal_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u8;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   1,    "177"),
            (   2,    "200"),
            (   3,    "377"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Octal));
        }

        Ok(())
    }

    #[test]
    fn test_octal_u16() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u16;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "0"),
            (   2,    "11064"),
            (   4,    "177777"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Octal));
        }

        Ok(())
    }

    #[test]
    fn test_octal_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u32;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "11064"),
            (   2,    "2215177777"),
            (   4,    "37777777777"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Octal));
        }

        Ok(())
    }

    #[test]
    fn test_octal_u64() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

        type TestType = u64;
        type TestEndian = BigEndian;

        let tests = vec![
            // index  expected
            (   0,    "443237777777777"),
        ];

        for (index, expected) in tests {
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Octal));
        }

        Ok(())
    }

    #[test]
    fn test_binary_i8() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\xab\xFF\xFF\xFF\xFF".to_vec();
        let context = Context::new(&data);

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<i8> = SizedInteger::<i8>::read(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Binary(BinaryOptions {
                padded: padded
            })));
        }

        Ok(())
    }

    #[test]
    fn test_scientific_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();
        let context = Context::new(&data);

        type TestType = u32;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Scientific( ScientificOptions {
                uppercase: uppercase,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_scientific_i32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();
        let context = Context::new(&data);

        type TestType = i32;
        type TestEndian = BigEndian;

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
            let mut context = context.clone();
            context.set_position(index);

            let t: SizedInteger<TestType> = SizedInteger::<TestType>::read::<TestEndian>(&context)?;
            assert_eq!(expected, t.to_string(IntegerDisplay::Scientific( ScientificOptions {
                uppercase: uppercase,
            })));
        }

        Ok(())
    }

    #[test]
    fn test_buffer_too_short() -> SimpleResult<()> {
        let data = b"".to_vec();
        assert!(SizedInteger::<u8>::read(&Context::new(&data)).is_err());

        let data = b"A".to_vec();
        assert!(SizedInteger::<u16>::read::<BigEndian>(&Context::new(&data)).is_err());

        let data = b"AAA".to_vec();
        assert!(SizedInteger::<u32>::read::<BigEndian>(&Context::new(&data)).is_err());

        let data = b"AAAAAAA".to_vec();
        assert!(SizedInteger::<u64>::read::<BigEndian>(&Context::new(&data)).is_err());

        let data = b"AAAAAAAAAAAAAAA".to_vec();
        assert!(SizedInteger::<u128>::read::<BigEndian>(&Context::new(&data)).is_err());

        Ok(())
    }
}
