use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct iCCP<'a> {
    name: &'a [u8],
    data: &'a [u8],
}

impl<'a> iCCP<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::iCCP  {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        let Some(mid) = chunk.into_iter().position(|&x| x == 0) else {
            return None;
        };
        let (name, data) = chunk.split_at(mid);
        Some(Self { name, data })
    }

    pub fn is_valid(&self) -> bool {
        // (only code points 0x20-7E and 0xA1-FF are allowed)
        self.name.into_iter().all(|&x| matches!(x, 0x20..=0x7E | 0xA1..=0xFF)) &&
        // Leading, trailing, and consecutive spaces are not permitted.
        // self.name.strip_prefix(&[0x20]).is_none() &&
        // self.name.strip_suffix(&[0x20]).is_none() &&
        self.name.split(|&x| x == 0x20).any(|x| x.is_empty()) &&
        self.data[0] == 0 &&
        self.data[1] == 0
    }
}
impl Display for iCCP<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{id: {}, name: {:?}, data: [..]}}",
        ChunkId::iCCP, String::from_iter(self.name.into_iter().map(|&x| x as char))))
    }
}
