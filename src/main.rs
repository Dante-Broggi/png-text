use std::{io::{Read, Write, self}, fmt::{Display, Debug}, collections::HashMap};

use clap::Parser;
use clio::*;

mod crc;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value="-")]
    input: Input,

    /// Output file '-' for stdout
    #[clap(long, short, value_parser, default_value="-")]
    output: Output,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let iter = args.input.bytes();
    // let d = iter.map(|x| (x, x)).unzip();
    parse_pngs(iter, args.output)?;
    // io::copy(&mut args.input, &mut args.output).unwrap();
    Ok(())
}

/*

iter all start bytes, if we fail to decode a chunk, continue
if we found a chunk, output it & start index, continue

 */



fn parse_pngs(bytes: io::Bytes<clio::Input>, mut stdinfo: clio::Output) -> io::Result<()> {
    let iter = bytes.filter_map(|x| x.ok());
    let iter: Vec<u8> = iter.collect();
    let slice = iter.as_slice();
    let mut magics: Vec<usize> = vec![];
    let mut chunks: HashMap<usize, Chunk> = HashMap::new();
    let mut next_chunk: HashMap<usize, usize> = HashMap::new();
    for start in 0..iter.len() {
        // writeln!(stdinfo, "testing offset: {} of {}", start, iter.len())?;
        let iter = slice.into_iter().cloned().skip(start);
        let iter2 = slice.into_iter().cloned().skip(start);
        if let Ok(_) = parse_png_magic(iter) {
            magics.push(start);
            // writeln!(stdinfo, "found PNG magic at offset: {}", start)?;
        } else if let Ok(c) = parse_png_chunk(iter2) {
            next_chunk.insert(start, start + c.full_len());
        // writeln!(stdinfo, "found chunk {{{}, len: {}}} at offset: {}", c.id, c.data.len(), start)?;
            chunks.insert(start, c);
        } else {
            continue;
        }
    }
    for mag in magics {
        let mut chunk = mag + PNG_MAGIC.len();
        writeln!(stdinfo, "found PNG magic at offset: {}, start chunk at: {}", mag, chunk)?;
        'chain: while let Some(c) = next_chunk.get(&chunk) {
            if let Some(ch) = chunks.remove(&chunk) {
                writeln!(stdinfo, "found chunk {{{}, len: {}}} at offset: {}", ch.id, ch.data.len(), chunk)?;                    
                chunk = *c;
                if ch.id == ChunkId::IEND {
                    writeln!(stdinfo, "^ ending chunk chain ^\n")?;
                    break 'chain;
                }
            } else if *c == slice.len() {
                writeln!(stdinfo, "^ ending chunk chain at EOF ^\n")?;
            } else {
                writeln!(stdinfo, "^ ending chunk chain ^\n")?;
            }
        }
    }
    for (off, c) in chunks {
        writeln!(stdinfo, "found bare chunk chain {{{}, len: {}}} at offset: {}", c.id, c.data.len(), off)?;
    }
    // writeln!(stdinfo, "<< end parse pngs")?;
    Ok(())
}

fn parse_png_iter(mut iter: core::iter::Skip<core::iter::Enumerate<impl Iterator<Item = u8>>>, mut stdinfo: impl Write) -> io::Result<()> {
    parse_png_magic(iter.by_ref().map(|x| x.1))?;
    let mut found_iend = false;
    while let Ok(x) = parse_png_chunk(iter.by_ref().map(|x| x.1)) {
        writeln!(stdinfo, "parsed chunk: {:?}", x.id)?;
        if x.id == ChunkId::IEND {
            found_iend = true;
            break;
        }
    }
    loop {
        let Ok((len, chunk)) = parse_png_chunk_head(iter.by_ref().map(|x| x.1).by_ref()) else {
            continue;
        };
        writeln!(stdinfo, "possible trailing header: {}, len: {}", chunk, len)?;
    }
    if let Some((_, b)) = iter.next() {
        writeln!(stdinfo, "remaining bytes:")?;
        write!(stdinfo, "{}", b)?;
    }
    for (_, _b) in iter {
        // write!(stdinfo, "{}", b?)?;
    }

    Ok(())
}
const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

fn parse_png_magic(iter: impl Iterator<Item = u8>) -> io::Result<()> {
    let mut px: usize = 0;
    for b in iter {
        // write!(stdinfo, "[{:?} | {:?}]", *b.as_ref().unwrap() as char, (b.as_ref().unwrap() & 0b0111_1111) as char)?;
        if b == PNG_MAGIC[px] {
            px += 1;
        } else {
            return Err(MyErr::InvalidData.into());
            // px = 0;
        }
        if px == PNG_MAGIC.len() {
            // writeln!(stdinfo, "Found PNG magic at index {:?}:", (ix + 1) - PNG_MAGIC.len())?;
            break;
        }
    }
    Ok(())
}
#[derive(Debug)]
enum MyErr {
    EOF, LargeLen, InvalidData,
}
impl Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::EOF => f.write_str("EOF"),
            MyErr::LargeLen => f.write_str("Too large chunk len"),
            MyErr::InvalidData => f.write_str("invalid PNG chunk"),
        }
    }
}
impl From<MyErr> for io::Error {
    fn from(value: MyErr) -> Self {
        match value {
            MyErr::EOF => Self::new(io::ErrorKind::UnexpectedEof, value),
            MyErr::LargeLen => Self::new(io::ErrorKind::InvalidData, value),
            MyErr::InvalidData => Self::new(io::ErrorKind::InvalidData, value),
        }
    }
}
impl std::error::Error for MyErr {

}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChunkId(pub [u8; 4]);
impl ChunkId {
    const DATA_BIT: u8 = 32;
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
        // for c in self.0.map(|x| x as char) {
        //     f.write_fmt(format_args!("{:?}", c))?;
        // }
        f.write_fmt(format_args!("{:?}", self.0.map(|x| x as char)))?;
        Ok(())
    }
}

fn parse_png_chunk_head(iter: &mut impl Iterator<Item = u8>) -> std::result::Result<(u32, ChunkId), MyErr> {
    let b = iter.next().ok_or(MyErr::EOF)?;
    let mut len: u32 = b.into();
    for _ in 0..3 {
        len <<= 8;
        let ok_or = iter.next().ok_or(MyErr::EOF)?;
        len |= ok_or as u32;
    }
    let mut chunk = [0; 4];
    for i in 0..4 {
        let ok_or = iter.next().ok_or(MyErr::EOF)?;
        chunk[i] = ok_or;
    }
    let chunk = ChunkId(chunk);
    if len > (i32::MAX as u32) {
        // writeln!(stdinfo, "too large chunk: {}, at byte: {}, len: {}", chunk, ix, len)?;
        return Err(MyErr::LargeLen)
    }
    if !chunk.is_valid() {
        // writeln!(stdinfo, "invalid chunk: {}, at byte: {}, len: {}", chunk, ix, len)?;
        return Err(MyErr::InvalidData)
    }
    // writeln!(stdinfo, "found chunk {}, at byte: {}, len: {}", chunk, ix, len)?;
    Ok((len, chunk))
}

#[derive(Debug)]
pub struct Chunk {
    id: ChunkId,
    data: Vec<u8>,
    crc: u32,
}
impl Chunk {
    /// does the crc match the data?
    pub fn is_valid(&self) -> bool {
        self.id.is_valid() &&
        crate::crc::crc(self.id, &self.data) == self.crc
    }
    pub fn full_len(&self) -> usize {
        4 + 4 + self.data.len() + 4
    }
}

fn parse_png_chunk(mut iter: impl Iterator<Item = u8>) -> std::result::Result<Chunk, MyErr> {
    let (len, chunk) = parse_png_chunk_head(iter.by_ref().map(|x| x).by_ref())?;
    let bytes: Vec<u8> = iter.by_ref().take(len as usize).collect();
    let mut crc: u32 = 0;
    for _ in 0..4 {
        crc <<= 8;
        let ok_or = iter.next().ok_or(MyErr::EOF)?;
        crc |= ok_or as u32;
    }
    let chunk = Chunk { id: chunk, data: bytes, crc };
    if !chunk.is_valid() {
        return Err(MyErr::InvalidData);
    }
    Ok(chunk)
}

