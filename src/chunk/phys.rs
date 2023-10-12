use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct pHYs<'a> {
    data: &'a [u8; 9],
}

impl<'a> pHYs<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::pHYs  {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        if chunk.len() == 9 {
            Some(Self { data: chunk.try_into().unwrap() })
        } else {
            None
        }
    }

    pub fn ppx(&self) -> u32 {
        u32::from_be_bytes(self.data[..4].try_into().unwrap())
    }
    pub fn ppy(&self) -> u32 {
        u32::from_be_bytes(self.data[4..8].try_into().unwrap())
    }
    pub fn unit(&self) -> u8 {
        self.data[8]
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.unit(), 0 | 1)
    }
}
impl Display for pHYs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit() {
            0 => {
                f.write_fmt(format_args!("{{id: {}, pixels_per_x: {}, pixels_per_y: {}}}",
                ChunkId::pHYs, self.ppx(), self.ppy()))
            },
            1 => {
                f.write_fmt(format_args!("{{id: {}, pixels_per_metre_x: {}, pixels_per_metre_y: {}}}",
                ChunkId::pHYs, self.ppx(), self.ppy()))
            },
            _ => {
                f.write_fmt(format_args!("{{id: {}, pixels_per_x: {}, pixels_per_y: {}, unit: {}, }}",
                ChunkId::pHYs, self.ppx(), self.ppy(), self.unit()))
            },
        }
    }
}
