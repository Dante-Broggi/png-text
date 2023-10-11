use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct tIME<'a> {
    data: &'a [u8],
}

impl<'a> tIME<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::tIME  {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        if chunk.len() == 7 {
            Some(Self { data: chunk.try_into().unwrap() })
        } else {
            None
        }
    }

    pub fn year(&self) -> u16 {
        u16::from_be_bytes(self.data[..2].try_into().unwrap())
    }
    pub fn month(&self) -> u8 {
        self.data[2]
    }
    pub fn day(&self) -> u8 {
        self.data[3]
    }
    pub fn hour(&self) -> u8 {
        self.data[4]
    }
    pub fn minute(&self) -> u8 {
        self.data[5]
    }
    pub fn second(&self) -> u8 {
        self.data[6]
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.month(), 1..=12) &&
        matches!(self.day(), 1..=31) &&
        matches!(self.hour(), 0..=23) &&
        matches!(self.minute(), 0..=59) &&
        matches!(self.second(), 0..=60)
    }
}
impl Display for tIME<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{id: {}, year: {}, month: {}, day: {}, hour: {}, minute: {}, second: {}}}",
        ChunkId::tIME, self.year(), self.month(), self.day(), self.hour(), self.minute(), self.second()))
    }
}
