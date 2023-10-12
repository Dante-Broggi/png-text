use core::fmt::{Display, Debug};

use self::{ihdr::IHDR, chrm::cHRM, gama::gAMA, iccp::iCCP, time::tIME, phys::pHYs};

mod ihdr;
mod chrm;
mod gama;
mod iccp;
mod time;
mod phys;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChunkId(pub [u8; 4]);
#[allow(non_upper_case_globals)]
impl ChunkId {
    const DATA_BIT: u8 = 32;
    pub const IHDR: Self = ChunkId(*b"IHDR");
    pub const PLTE: Self = ChunkId(*b"PLTE");
    pub const IDAT: Self = ChunkId(*b"IDAT");
    pub const tRNS: Self = ChunkId(*b"tRNS");
    pub const cHRM: Self = ChunkId(*b"cHRM");
    pub const gAMA: Self = ChunkId(*b"gAMA");
    pub const iCCP: Self = ChunkId(*b"iCCP");
    pub const sBIT: Self = ChunkId(*b"sBIT");
    pub const tIME: Self = ChunkId(*b"tIME");
    pub const pHYs: Self = ChunkId(*b"pHYs");
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


#[derive(Debug, PartialEq, Eq)]
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
                let Some(ihdr) = IHDR::new(self) else {
                    return false;
                };
                ihdr.is_valid()
            },
            ChunkId::PLTE => {
                self.data.len() % 3 == 0 &&
                self.data.len() > 0
            },
            ChunkId::IDAT => {
                true
            },
            ChunkId::tRNS => {
                true
            },
            ChunkId::cHRM => {
                let Some(chrm) = cHRM::new(self) else {
                    return false;
                };
                chrm.is_valid()
            },
            ChunkId::gAMA => {
                let Some(gama) = gAMA::new(self) else {
                    return false;
                };
                gama.is_valid()
            },
            ChunkId::iCCP => {
                let Some(iccp) = iCCP::new(self) else {
                    return false;
                };
                iccp.is_valid()
            },
            ChunkId::sBIT => {
                true
            },
            ChunkId::tIME => {
                let Some(time) = tIME::new(self) else {
                    return false;
                };
                time.is_valid()
            },
            ChunkId::pHYs => {
                let Some(phys) = pHYs::new(self) else {
                    return false;
                };
                phys.is_valid()
            },
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
                    Display::fmt(&IHDR::new(self).unwrap(), f)?;
                },
                ChunkId::PLTE => {
                    f.write_fmt(format_args!("{{id: {}, palette_len: {}}}", self.id, self.data.len()/3))?;
                },
                ChunkId::IEND => {
                    f.write_fmt(format_args!("{{id: {}}}", self.id))?;
                },
                ChunkId::tRNS => {
                    f.write_fmt(format_args!("{{id: {}, len:{}}}", self.id, self.data.len()))?;
                },
                ChunkId::cHRM => {
                    Display::fmt(&cHRM::new(self).unwrap(), f)?;
                },
                ChunkId::gAMA => {
                    Display::fmt(&gAMA::new(self).unwrap(), f)?;
                },
                ChunkId::IDAT => {
                    f.write_fmt(format_args!("{{id: {}, len:{}}}", self.id, self.data.len()))?;
                },
                ChunkId::iCCP => {
                    Display::fmt(&iCCP::new(self).unwrap(), f)?;
                },
                ChunkId::sBIT => {
                    f.write_fmt(format_args!("{{id: {}, len:{}}}", self.id, self.data.len()))?;
                },
                ChunkId::tIME => {
                    Display::fmt(&tIME::new(self).unwrap(), f)?;
                },
                ChunkId::pHYs => {
                    Display::fmt(&pHYs::new(self).unwrap(), f)?;
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
