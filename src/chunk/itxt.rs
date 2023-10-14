use core::fmt::{Display, Debug};

use super::ChunkId;

#[allow(non_camel_case_types)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct iTXt<'a> {
    keyword: &'a [u8],
    // Null separator	1 byte (null character)
    cflg: u8,
    cmthd: u8,
    lang: &'a [u8],
    // Null separator	1 byte (null character)
    trans: &'a str,
    // Null separator	1 byte (null character)
    /// Not `str` if compressed:
    text: &'a [u8],

/*
Keyword	1-79 bytes (character string)
Null separator	1 byte (null character)
Compression flag	1 byte
Compression method	1 byte
Language tag	0 or more bytes (character string)
Null separator	1 byte (null character)
Translated keyword	0 or more bytes
Null separator	1 byte (null character)
Text	0 or more bytes
*/
}

impl<'a> iTXt<'a> {
    pub fn new(chunk: &'a super::Chunk) -> Option<Self> {
        if chunk.is_valid_chunk() && chunk.id == ChunkId::iTXt  {
            Self::new_slice(&chunk.data)
        } else {
            None
        }
    }
    pub fn new_slice(chunk: &'a [u8]) -> Option<Self> {
        let Some(nx) = chunk.into_iter().position(|&x| x == 0) else {
            return None;
        };
        let (keyword, chunk) = chunk.split_at(nx);
        let (&n, chunk) = chunk.split_first().unwrap();
        assert_eq!(n, 0);
        let Some((&cflg, chunk)) = chunk.split_first() else {
            return None;
        };
        let Some((&cmthd, chunk)) = chunk.split_first() else {
            return None;
        };
        let Some(nx) = chunk.into_iter().position(|&x| x == 0) else {
            return None;
        };
        let (lang, chunk) = chunk.split_at(nx);
        let (&n, chunk) = chunk.split_first().unwrap();
        assert_eq!(n, 0);
        let Some(nx) = chunk.into_iter().position(|&x| x == 0) else {
            return None;
        };
        let (trans, chunk) = chunk.split_at(nx);
        let (&n, chunk) = chunk.split_first().unwrap();
        assert_eq!(n, 0);
        let trans = core::str::from_utf8(trans).ok()?;

        Some(Self {
            keyword,
            cflg,
            cmthd,
            lang,
            trans,
            text: chunk,
        })
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.cflg, 0 | 1)
        // ...
    }
}
impl Display for iTXt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cflg == 0 {
            if let Ok(text) = core::str::from_utf8(self.text) {
                if self.cmthd == 0 {
                    f.write_fmt(format_args!("{{id: {}, keyword: {:?}, lang: {:?}, trans_keyword: {:?}, text: {:?}}}",
                    ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)),
                    String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                    self.trans, text))
                } else {
                    f.write_fmt(format_args!("{{id: {}, keyword: {:?}, raw_cmthd: {}, lang: {:?}, trans_keyword: {:?}, text: {:?}}}",
                    ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)), self.cmthd,
                    String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                    self.trans, text))
                }
            } else {
                if self.cmthd == 0 {
                    f.write_fmt(format_args!("{{id: {}, keyword: {:?}, lang: {:?}, trans_keyword: {:?}, text: {:?}}}",
                    ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)),
                    String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                    self.trans, self.text))
                } else {
                    f.write_fmt(format_args!("{{id: {}, keyword: {:?}, raw_cmthd: {}, lang: {:?}, trans_keyword: {:?}, text: {:?}}}",
                    ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)), self.cmthd,
                    String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                    self.trans, self.text))
                }
            }
        } else {
            if self.cmthd == 0 {
                f.write_fmt(format_args!("{{id: {}, keyword: {:?}, lang: {:?}, trans_keyword: {:?}, text: [..]}}",
                ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)),
                String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                self.trans))
            } else {
                f.write_fmt(format_args!("{{id: {}, keyword: {:?}, compression_method: {}, lang: {:?}, trans_keyword: {:?}, text: [..]}}",
                ChunkId::iTXt, String::from_iter(self.keyword.into_iter().map(|&x| x as char)), self.cmthd,
                String::from_iter(self.lang.into_iter().map(|&x| x as char)),
                self.trans))
            }
        }
    }
}
