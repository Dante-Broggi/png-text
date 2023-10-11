use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct cHRM<'a> {
    data: &'a [u8; 2 * 4 * 4],
}

impl<'a> cHRM<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::cHRM {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        if chunk.len() == (2 * 4 * 4) {
            Some(Self { data: chunk.try_into().unwrap() })
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
    pub fn white_point(&self) -> (u32, u32) {
        (u32::from_be_bytes(self.data[..4].try_into().unwrap()),
        u32::from_be_bytes(self.data[4..8].try_into().unwrap()))
    }
    pub fn red(&self) -> (u32, u32) {
        (u32::from_be_bytes(self.data[8..12].try_into().unwrap()),
        u32::from_be_bytes(self.data[12..16].try_into().unwrap()))
    }
    pub fn green(&self) -> (u32, u32) {
        (u32::from_be_bytes(self.data[16..20].try_into().unwrap()),
        u32::from_be_bytes(self.data[20..24].try_into().unwrap()))
    }
    pub fn blue(&self) -> (u32, u32) {
        (u32::from_be_bytes(self.data[24..28].try_into().unwrap()),
        u32::from_be_bytes(self.data[28..32].try_into().unwrap()))
    }
}
impl Display for cHRM<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: div components by 100000
        f.write_fmt(format_args!("{{id: {}, white_point: {:?}, red: {:?}, green: {:?}, blue: {:?}}}",
        ChunkId::cHRM, self.white_point(), self.red(), self.green(), self.blue()))
    }
}
