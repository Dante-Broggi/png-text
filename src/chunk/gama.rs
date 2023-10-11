use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct gAMA<'a> {
    data: &'a [u8; 4],
}

impl<'a> gAMA<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::gAMA {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        if chunk.len() == 4 {
            Some(Self { data: chunk.try_into().unwrap() })
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
    pub fn gamma(&self) -> u32 {
        u32::from_be_bytes(*self.data)
    }
}
impl Display for gAMA<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: div components by 100000
        f.write_fmt(format_args!("{{id: {}, gamma: {:?}}}",
        ChunkId::gAMA, self.gamma(), ))
    }
}
