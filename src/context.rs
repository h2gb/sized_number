use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use simple_error::{SimpleError, SimpleResult, bail};
use std::io::{Cursor, Read};

#[cfg(feature = "serialize")]
use serde::{Serialize, Deserialize};

/// Define the endianness for reading multi-byte integers
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Endian {
    /// Most significant byte is first (eg, `0x1234` -> `12 34`)
    Big,

    /// Most significant byte is last (eg, `0x1234` -> `34 12`)
    Little,
}

/// A structure to hold a data structure and a position while reading the data.
///
/// This is essentially a [`Cursor`], but with some convenience functions to
/// clone and set the position more quickly.
#[derive(Debug, Clone)]
pub struct Context<'a> {
    c: Cursor<&'a Vec<u8>>,
}

impl<'a> Context<'a> {
    /// Create a new [`Context`] at position 0.
    ///
    /// Cannot fail, even if the Vec is empty.
    pub fn new(v: &'a Vec<u8>) -> Self {
        Self {
            c: Cursor::new(v)
        }
    }

    /// Create a new [`Context`] at a given position.
    ///
    /// Cannot fail, even if the Vec is empty or if the index is crazy. Those
    /// are checked when using the cursor, not while creating it.
    pub fn new_at(v: &'a Vec<u8>, index: u64) -> Self {
        let mut c = Cursor::new(v);
        c.set_position(index);

        Self {
            c: c
        }
    }

    /// Return a clone of the Cursor.
    ///
    /// This is for internal use only. We clone a lot while reading values, but
    /// this operation is reasonably inexpensive since we don't actually clone
    /// the data - just a reference.
    fn cursor(&self) -> Cursor<&Vec<u8>> {
        self.c.clone()
    }

    /// Clone the [`Context`] and change the position at the same time.
    ///
    /// I found myself doing a clone-then-set-position operation a bunch, so
    /// this simplifies it.
    pub fn at(&self, new_position: u64) -> Self {
        let mut c = self.clone();
        c.c.set_position(new_position);

        c
    }

    /// Get the current position.
    pub fn position(&self) -> u64 {
        self.c.position()
    }

    pub fn read_u8(&self) -> SimpleResult<u8> {
        match self.cursor().read_u8() {
            Ok(i) => Ok(i),
            Err(e) => Err(SimpleError::from(e)),
        }
    }

    pub fn read_u16(&self, endian: Endian) -> SimpleResult<u16> {
        match endian {
            Endian::Big => match self.cursor().read_u16::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_u16::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_u32(&self, endian: Endian) -> SimpleResult<u32> {
        match endian {
            Endian::Big => match self.cursor().read_u32::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_u32::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_u64(&self, endian: Endian) -> SimpleResult<u64> {
        match endian {
            Endian::Big => match self.cursor().read_u64::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_u64::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_u128(&self, endian: Endian) -> SimpleResult<u128> {
        match endian {
            Endian::Big => match self.cursor().read_u128::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_u128::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_i8(&self) -> SimpleResult<i8> {
        match self.cursor().read_i8() {
            Ok(i) => Ok(i),
            Err(e) => Err(SimpleError::from(e)),
        }
    }

    pub fn read_i16(&self, endian: Endian) -> SimpleResult<i16> {
        match endian {
            Endian::Big => match self.cursor().read_i16::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_i16::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_i32(&self, endian: Endian) -> SimpleResult<i32> {
        match endian {
            Endian::Big => match self.cursor().read_i32::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_i32::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_i64(&self, endian: Endian) -> SimpleResult<i64> {
        match endian {
            Endian::Big => match self.cursor().read_i64::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_i64::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_i128(&self, endian: Endian) -> SimpleResult<i128> {
        match endian {
            Endian::Big => match self.cursor().read_i128::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_i128::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_f32(&self, endian: Endian) -> SimpleResult<f32> {
        match endian {
            Endian::Big => match self.cursor().read_f32::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_f32::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_f64(&self, endian: Endian) -> SimpleResult<f64> {
        match endian {
            Endian::Big => match self.cursor().read_f64::<BigEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
            Endian::Little => match self.cursor().read_f64::<LittleEndian>() {
                Ok(i) => Ok(i),
                Err(e) => Err(SimpleError::from(e)),
            },
        }
    }

    pub fn read_bytes(&self, size: usize) -> SimpleResult<Vec<u8>> {
        let mut v: Vec<u8> = Vec::with_capacity(size);

        match self.cursor().take(size as u64).read_to_end(&mut v) {
            Ok(read_size) => {
                if read_size < size {
                    bail!("Read past end of buffer");
                }

                Ok(v)
            }
            Err(e) => Err(SimpleError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;

    #[test]
    fn test_read_bytes() -> SimpleResult<()> {
        // Most functionality on context is implicitly exercised by the tests
        // in lib.rs, but read_bytes is not so test it
        let data = b"ABCD".to_vec();

        // Valid
        assert_eq!(b"ABCD".to_vec(), Context::new(&data).read_bytes(4)?);
        assert_eq!(b"ABC".to_vec(), Context::new(&data).read_bytes(3)?);
        assert_eq!(b"BCD".to_vec(), Context::new_at(&data, 1).read_bytes(3)?);
        assert_eq!(b"".to_vec(), Context::new(&data).read_bytes(0)?);

        // Technically, we can read 0 bytes from way off the buffer.. I think
        // that's okay?
        assert_eq!(b"".to_vec(), Context::new_at(&data, 1000).read_bytes(0)?);


        // Not valid
        assert!(Context::new(&data).read_bytes(5).is_err());
        assert!(Context::new_at(&data, 5).read_bytes(1).is_err());

        Ok(())
    }
}

