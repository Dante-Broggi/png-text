use core::fmt::{Display, Debug};

use super::ChunkId;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IHDR<'a> {
    data: &'a [u8; 13],
}

impl<'a> IHDR<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == super::ChunkId::IHDR && chunk.data.len() == 13 {
            Some(Self { data: chunk.data.as_slice().try_into().unwrap() })
        } else {
            None
        }
    }
    /*
Width	4 bytes
Height	4 bytes
Bit depth	1 byte
Colour type	1 byte
Compression method	1 byte
Filter method	1 byte
Interlace method	1 byte
    */
    pub fn width(&self) -> u32 {
        u32::from_be_bytes(self.data[..4].try_into().unwrap())
    }
    pub fn height(&self) -> u32 {
        u32::from_be_bytes(self.data[4..8].try_into().unwrap())
    }
    pub fn bit_depth(&self) -> u8 {
        self.data[8]
    }
    pub fn colour_type(&self) -> u8 {
        self.data[9]
    }
    fn colour_type_str(&self) -> &str {
        match self.colour_type() {
            0 => "Greyscale",
            2 => "Truecolour",
            3 => "Indexed",
            4 => "Greyscale_with_alpha",
            6 => "Truecolour_with_alpha",
            _ => panic!()
        }
    }
    pub fn compression_method(&self) -> u8 {
        self.data[10]
    }
    pub fn filter_method(&self) -> u8 {
        self.data[11]
    }
    pub fn interlace_method(&self) -> u8 {
        self.data[12]
    }
    fn interlace_method_str(&self) -> &str {
        match self.interlace_method() {
            0 => "",
            1 => "interlace: Adam7, ",
            _ => panic!()
        }
    }
    pub fn is_valid(self) -> bool {
        if self.width() != 0 && self.height() != 0 &&
        matches!(self.bit_depth(), 1 | 2 | 4 | 8 | 16) &&
        matches!(self.colour_type(), 0 | 2 | 3 | 4 | 6) &&
        matches!(self.compression_method(), 0) &&
        matches!(self.filter_method(), 0) &&
        matches!(self.interlace_method(), 0 | 1) {
            // https://www.w3.org/TR/png-3/#table111
            match self.colour_type() {
                0 => {
                    matches!(self.bit_depth(), 1 | 2 | 4 | 8 | 16)
                },
                3 => {
                    matches!(self.bit_depth(), 1 | 2 | 4 | 8)
                },
                2 | 4 | 6 => {
                    matches!(self.bit_depth(), 8 | 16)
                },
                _ => false,
            }
        } else {
            false
        }
    }

}
impl Display for IHDR<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            f.write_fmt(format_args!("{{id:{}, width: {   }, height: {   }, bit_depth: {   }, colour_type: {   }, {}..}}",
                                ChunkId::IHDR, self.width(), self.height(), self.bit_depth(), self.colour_type_str(), self.interlace_method_str()))
        } else {
            f.write_fmt(format_args!("{{id:{}, width: {}, height: {}, bit_depth: {}, colour_type: {}, compression_method: {}, filter_method: {}, interlace: {}}}",
                                     ChunkId::IHDR, self.width(), self.height(), self.bit_depth(), self.colour_type(), self.compression_method(), self.filter_method(), self.interlace_method()))
        }
    }
}
