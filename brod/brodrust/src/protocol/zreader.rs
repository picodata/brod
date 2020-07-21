use std::str;

use byteorder::{BigEndian, ByteOrder};
use crate::kafka_error::{Result, ErrorKind};

static EMPTY_STR: &'static str = "";

pub struct ZReader<'a> {
    data: &'a [u8],
}

// ~ a helper macro to hide away the used byte order
macro_rules! dec {
    ($method:ident, $src:expr) => { BigEndian::$method($src) }
}

impl<'a> ZReader<'a> {
    pub fn new(data: &[u8]) -> ZReader {
        ZReader { data: data }
    }

    /// ~ Consumes `n_bytes` from the underlying slice while returning
    /// the consumed bytes. The returned slice is guaranteed to be
    /// `n_bytes` long. This operation either succeeds or fails as a
    /// whole. Upon failure the reader will _not_ advance.
    pub fn read<'b>(&'b mut self, n_bytes: usize) -> Result<&'a [u8]> {
        if n_bytes > self.data.len() {
            bail!(ErrorKind::UnexpectedEOF)
        } else {
            let (x, rest) = self.data.split_at(n_bytes);
            self.data = rest;
            Ok(x)
        }
    }

    /// ~ Retrieves the rest of the underlying slice without advancing
    /// this reader.
    pub fn rest(&self) -> &[u8] {
        self.data
    }

    /// ~ Determines whether there are still some bytes available for
    /// consumption.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.read(1).map(|x| unsafe { *x.get_unchecked(0) as i8 })
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        self.read(2).map(|x| dec!(read_i16, x))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        self.read(4).map(|x| dec!(read_i32, x))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        self.read(8).map(|x| dec!(read_i64, x))
    }

    /// Reads a string as defined by the Kafka Protocol. The 'null'
    /// string is delivered as the empty string.
    pub fn read_str<'b>(&'b mut self) -> Result<&'a str> {
        let len = self.read_i16()?;
        if len <= 0 {
            Ok(EMPTY_STR)
        } else {
            // alternatively: str::from_utf8_unchecked(..)
            match str::from_utf8(self.read(len as usize)?) {
                Ok(s) => Ok(s),
                Err(_) => bail!(ErrorKind::StringDecodeError),
            }
        }
    }

    /// Reads 'bytes' as defined by the Kafka Protocol. The 'null'
    /// bytes are delivered as an empty slice.
    pub fn read_bytes<'b>(&'b mut self) -> Result<&'a [u8]> {
        let len = self.read_i32()?;
        if len <= 0 {
            Ok(&self.data[0..0])
        } else {
            self.read(len as usize)
        }
    }

    /// Reads the size of an array as defined by the Kafka
    /// Protocol. The size of 'null' array will be returned as the
    /// size an array of an empty array.
    pub fn read_array_len(&mut self) -> Result<usize> {
        let len = self.read_i32()?;
        Ok(if len < 0 { 0 } else { len as usize })
    }
}
