use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct sRGB {
    rendering_intent: u8,
}

impl sRGB {
    pub fn new(chunk: &'_ super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::sRGB  {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'_ [u8]) -> Option<Self> {
        if chunk.len() == 1 {
            Some(Self { rendering_intent: chunk[0] })
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        self.rendering_intent < 4
    }
    fn rendering_intent_str(&self) -> &str {
        match self.rendering_intent {
            0 => "Perceptual",
            1 => "Relative colorimetric",
            2 => "Saturation",
            3 => "Absolute colorimetric",
            _ => panic!(),
        }
    }
}
impl Display for sRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            f.write_fmt(format_args!("{{id: {}, rendering_intent: {}}}",
            ChunkId::sRGB, self.rendering_intent_str()))
        } else {
            f.write_fmt(format_args!("{{id: {}, rendering_intent: {}}}",
            ChunkId::sRGB, self.rendering_intent))
        }
    }
}
