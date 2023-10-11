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
