use core::fmt::{Display, Debug};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChunkId(pub [u8; 4]);
impl ChunkId {
    const DATA_BIT: u8 = 32;
    pub const IHDR: Self = ChunkId(*b"IHDR");
    pub const PLTE: Self = ChunkId(*b"PLTE");
    pub const IEND: Self = ChunkId(*b"IEND");

    /// Each byte of a chunk type is restricted to
    /// the hexadecimal values 41 to 5A and 61 to 7A.
    pub fn is_valid(self) -> bool {
        self.0.iter().all(|x| (0x41..=0x5A).contains(x) || (0x61..=0x7A).contains(x))
    }

    pub fn is_display_critical(self) -> bool {
        (self.0[0] & Self::DATA_BIT) == 0
    }
    pub fn is_standard_defined(self) -> bool {
        (self.0[1] & Self::DATA_BIT) == 0
    }
    pub fn is_reserved(self) -> bool {
        (self.0[2] & Self::DATA_BIT) != 0
    }
    pub fn is_safe_to_copy(self) -> bool {
        (self.0[3] & Self::DATA_BIT) != 0
    }
}
impl Debug for ChunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for c in self.0.map(|x| x as char) {
        //     f.write_fmt(format_args!("{:?}", c))?;
        // }
        f.write_fmt(format_args!("{:?}", self.0.map(|x| x as char)))?;
        Ok(())
    }
}
impl Display for ChunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = self.0.map(|x| x as char);
        let s = String::from_iter(iter);
        f.write_fmt(format_args!("{:?}", s))?;
        Ok(())
    }
}


#[derive(Debug)]
pub struct Chunk {
    pub(crate) id: ChunkId,
    pub(crate) data: Vec<u8>,
    pub(crate) crc: u32,
}
impl Chunk {
    /// does the crc match the data and is it a valid id?
    pub fn is_valid_chunk(&self) -> bool {
        self.id.is_valid() &&
        crate::crc::crc(self.id, &self.data) == self.crc
    }
    /// does the chunk match the spec?
    pub fn is_valid_spec(&self) -> bool {
        if !self.is_valid_chunk() { return false; }
        match self.id {
            ChunkId::IHDR => {
                if self.data.len() == 13 &&
                self.data[..8].into_iter().any(|&x| x != 0) &&
                matches!(self.data[8], 1 | 2 | 4 | 8 | 16) &&
                matches!(self.data[9], 0 | 2 | 3 | 4 | 6) &&
                matches!(self.data[10], 0) &&
                matches!(self.data[11], 0) &&
                matches!(self.data[12], 0 | 1) {
                    // https://www.w3.org/TR/png-3/#table111
                    match self.data[9] {
                        0 => {
                            matches!(self.data[8], 1 | 2 | 4 | 8 | 16)
                        },
                        3 => {
                            matches!(self.data[8], 1 | 2 | 4 | 8)
                        },
                        2 | 4 | 6 => {
                            matches!(self.data[8], 8 | 16)
                        },
                        _ => false,
                    }
                } else {
                    false
                }

            },
            ChunkId::PLTE => {
                self.data.len() % 3 == 0 &&
                self.data.len() > 0
            }
            ChunkId::IEND => {
                self.data.len() == 0
            },
            _ => {
                true
            }
        }
    }
    pub fn full_len(&self) -> usize {
        4 + 4 + self.data.len() + 4
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid_spec() {
            match self.id {
                ChunkId::IHDR => {
                    f.write_fmt(format_args!("{{id:{}, width: {}, height: {}, bit_depth: {}, colour_type: {}, interlace: {}, ..}}",
                                             self.id, u32::from_be_bytes(self.data[..4].try_into().unwrap()), u32::from_be_bytes(self.data[4..8].try_into().unwrap()), self.data[8], self.data[9], self.data[12]))?;
                },
                ChunkId::IEND => {
                    f.write_fmt(format_args!("{{{}}}", self.id))?;
                },
                _ => {
                    f.write_fmt(format_args!("{{id: {}, len:{}}}", self.id, self.data.len()))?;
                }
            }
        } else {
            f.write_fmt(format_args!("{{id: {}, len:{}, crc: {:?}}}", self.id, self.data.len(), &self.crc.to_be_bytes()))?;
        }
        Ok(())
    }
}
