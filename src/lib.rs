use std::io;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum KError<'a> {
    InvalidContents { actual: &'a [u8] },
    IoError(io::Error),
    UnknownEnum(u64),
}

impl<'a> From<io::Error> for KError<'a> {
    fn from(e: io::Error) -> Self {
        KError::IoError(e)
    }
}

pub type KResult<'a, T> = Result<T, KError<'a>>;

pub trait KStruct<'a> {
    type Parent: KStruct<'a>;
    type Root: KStruct<'a>;

    /// Create a new instance of this struct; if we are the root node,
    /// then both `_parent` and `_root` will be `None`.
    fn new(_parent: Option<&'a Self::Parent>, _root: Option<&'a Self::Root>) -> KResult<'a, Self>
    where
        Self: Sized;

    fn read<S: KStream>(&mut self, stream: &mut S) -> KResult<'a, ()>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct KStructUnit<'a> {
    phantom: PhantomData<&'a ()>
}
impl<'a> KStruct<'a> for KStructUnit<'a> {
    type Parent = KStructUnit<'a>;
    type Root = KStructUnit<'a>;

    fn new(_parent: Option<&'a Self::Parent>, _root: Option<&'a Self::Root>) -> Result<Self, KError<'a>> where
        Self: Sized {
        Ok(KStructUnit { phantom: PhantomData })
    }

    fn read<S: KStream>(&mut self, _stream: &mut S) -> Result<(), KError<'a>> {
        Ok(())
    }
}

pub trait KStream {
    fn is_eof(&self) -> io::Result<bool>;
    fn seek(&mut self, position: u64) -> io::Result<()>;
    fn pos(&self) -> io::Result<u64>;
    fn size(&self) -> io::Result<u64>;

    fn read_s1(&mut self) -> io::Result<i8>;
    fn read_s2be(&mut self) -> io::Result<i16>;
    fn read_s4be(&mut self) -> io::Result<i32>;
    fn read_s8be(&mut self) -> io::Result<i64>;
    fn read_s2le(&mut self) -> io::Result<i16>;
    fn read_s4le(&mut self) -> io::Result<i32>;
    fn read_s8le(&mut self) -> io::Result<i64>;

    fn read_u1(&mut self) -> io::Result<u8>;
    fn read_u2be(&mut self) -> io::Result<u16>;
    fn read_u4be(&mut self) -> io::Result<u32>;
    fn read_u8be(&mut self) -> io::Result<u64>;
    fn read_u2le(&mut self) -> io::Result<u16>;
    fn read_u4le(&mut self) -> io::Result<u32>;
    fn read_u8le(&mut self) -> io::Result<u64>;

    fn read_f4be(&mut self) -> io::Result<f32>;
    fn read_f8be(&mut self) -> io::Result<f64>;
    fn read_f4le(&mut self) -> io::Result<f32>;
    fn read_f8le(&mut self) -> io::Result<f64>;

    fn align_to_byte(&mut self) -> io::Result<()>;
    fn read_bits_int(&mut self, n: u32) -> io::Result<u64>;

    fn read_bytes(&mut self, len: usize) -> io::Result<&[u8]>;
    fn read_bytes_full(&mut self) -> io::Result<&[u8]>;
    fn read_bytes_term(
        &mut self,
        term: char,
        include: bool,
        consume: bool,
        eos_error: bool,
    ) -> io::Result<&[u8]>;

    fn ensure_fixed_contents(&mut self, expected: &[u8]) -> KResult<&[u8]> {
        let actual = self.read_bytes(expected.len())?;
        if actual == expected {
            Ok(actual)
        } else {
            // Return what the actual contents were; our caller provided us
            // what was expected so we don't need to return it, and it makes
            // the lifetimes way easier
            Err(KError::InvalidContents { actual })
        }
    }

    /// Return a byte array that is sized to exclude all trailing instances of the
    /// padding character.
    fn bytes_strip_right(bytes: &[u8], pad: u8) -> &[u8] {
        let mut new_len = bytes.len();
        while new_len > 0 && bytes[new_len - 1] == pad {
            new_len -= 1;
        }
        &bytes[..new_len]
    }

    /// Return a byte array that contains all bytes up until the
    /// termination byte. Can optionally include the termination byte as well.
    fn bytes_terminate(bytes: &[u8], term: u8, include_term: bool) -> &[u8] {
        let mut new_len = 0;
        while bytes[new_len] != term && new_len < bytes.len() {
            new_len += 1;
        }

        if include_term && new_len < bytes.len() {
            new_len += 1;
        }

        &bytes[..new_len]
    }
}

#[allow(dead_code)]
struct BytesReader<'a> {
    bytes: &'a [u8],
    pos: usize,
    bits: u8,
    bits_left: u8,
}
impl<'a> BytesReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        BytesReader {
            bytes,
            pos: 0,
            bits: 0,
            bits_left: 0,
        }
    }
}
impl<'a> From<&'a [u8]> for BytesReader<'a> {
    fn from(b: &'a [u8]) -> Self {
        BytesReader::new(b)
    }
}
impl<'a> KStream for BytesReader<'a> {
    fn is_eof(&self) -> io::Result<bool> {
        unimplemented!()
    }

    fn seek(&mut self, _position: u64) -> io::Result<()> {
        unimplemented!()
    }

    fn pos(&self) -> io::Result<u64> {
        unimplemented!()
    }

    fn size(&self) -> io::Result<u64> {
        unimplemented!()
    }

    fn read_s1(&mut self) -> io::Result<i8> {
        unimplemented!()
    }

    fn read_s2be(&mut self) -> io::Result<i16> {
        unimplemented!()
    }

    fn read_s4be(&mut self) -> io::Result<i32> {
        unimplemented!()
    }

    fn read_s8be(&mut self) -> io::Result<i64> {
        unimplemented!()
    }

    fn read_s2le(&mut self) -> io::Result<i16> {
        unimplemented!()
    }

    fn read_s4le(&mut self) -> io::Result<i32> {
        unimplemented!()
    }

    fn read_s8le(&mut self) -> io::Result<i64> {
        unimplemented!()
    }

    fn read_u1(&mut self) -> io::Result<u8> {
        unimplemented!()
    }

    fn read_u2be(&mut self) -> io::Result<u16> {
        unimplemented!()
    }

    fn read_u4be(&mut self) -> io::Result<u32> {
        unimplemented!()
    }

    fn read_u8be(&mut self) -> io::Result<u64> {
        unimplemented!()
    }

    fn read_u2le(&mut self) -> io::Result<u16> {
        unimplemented!()
    }

    fn read_u4le(&mut self) -> io::Result<u32> {
        unimplemented!()
    }

    fn read_u8le(&mut self) -> io::Result<u64> {
        unimplemented!()
    }

    fn read_f4be(&mut self) -> io::Result<f32> {
        unimplemented!()
    }

    fn read_f8be(&mut self) -> io::Result<f64> {
        unimplemented!()
    }

    fn read_f4le(&mut self) -> io::Result<f32> {
        unimplemented!()
    }

    fn read_f8le(&mut self) -> io::Result<f64> {
        unimplemented!()
    }

    fn align_to_byte(&mut self) -> io::Result<()> {
        unimplemented!()
    }

    fn read_bits_int(&mut self, _n: u32) -> io::Result<u64> {
        unimplemented!()
    }

    fn read_bytes(&mut self, _len: usize) -> io::Result<&[u8]> {
        unimplemented!()
    }

    fn read_bytes_full(&mut self) -> io::Result<&[u8]> {
        unimplemented!()
    }

    fn read_bytes_term(
        &mut self,
        _term: char,
        _include: bool,
        _consume: bool,
        _eos_error: bool,
    ) -> io::Result<&[u8]> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_strip_right() {
        let b = [1, 2, 3, 4, 5, 5, 5, 5];
        let c = BytesReader::bytes_strip_right(&b, 5);

        assert_eq!([1, 2, 3, 4], c);
    }
}
