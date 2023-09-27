use std::{io::{Read, Write, Result, self}, fmt::{Display, Debug}};

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
    // std::io::copy(&mut args.input, &mut args.output).unwrap();
    Ok(())
}

/*

iter all start bytes, if we fail to decode a chunk, continue
if we found a chunk, output it & start index, continue

 */



fn parse_pngs(bytes: io::Bytes<clio::Input>, stdinfo: clio::Output) -> std::io::Result<()> {
    let iter = bytes.filter_map(|x| x.ok());
    let iter: Vec<u8> = iter.collect();
    parse_png_iter(iter.into_iter().enumerate(), stdinfo)
}

fn parse_png_iter(mut iter: core::iter::Enumerate<impl Iterator<Item = u8>>, mut stdinfo: impl Write) -> std::io::Result<()> {
    parse_png_magic(iter.by_ref(), &mut stdinfo)?;
    let mut found_iend = false;
    while let Ok(x) = parse_png_chunk(&mut iter, &mut stdinfo) {
        writeln!(stdinfo, "parsed chunk: {:?}", x.id)?;
        if x.id == ChunkId::IEND {
            found_iend = true;
            break;
        }
    }
    loop {
        let Ok((len, chunk)) = parse_png_chunk_head(&mut iter, &mut stdinfo) else {
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

fn parse_png_magic(iter: &mut impl Iterator<Item = (usize, u8)>, mut stdinfo: impl Write) -> std::io::Result<()> {
    const PNG: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let mut px: usize = 0;
    for (ix, b) in iter {
        // write!(stdinfo, "[{:?} | {:?}]", *b.as_ref().unwrap() as char, (b.as_ref().unwrap() & 0b0111_1111) as char)?;
        if b == PNG[px] {
            px += 1;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, Err::InvalidData));
            // px = 0;
        }
        if px == PNG.len() {
            writeln!(stdinfo, "Found PNG magic at index {:?}:", (ix + 1) - PNG.len())?;
            break;
        }
    }
    Ok(())
}
#[derive(Debug)]
enum Err {
    EOF, LargeLen, InvalidData,
}
impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::EOF => f.write_str("EOF"),
            Err::LargeLen => f.write_str("Too large chunk len"),
            Err::InvalidData => f.write_str("invalid PNG chunk"),
        }
    }
}
impl std::error::Error for Err {

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

fn parse_png_chunk_head(iter: &mut impl Iterator<Item = (usize, u8)>, mut stdinfo: impl Write) -> Result<(u32, ChunkId)> {
    let (ix, b) = iter.next().ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, Err::EOF))?;
    let mut len: u32 = b.into();
    for _ in 0..3 {
        len <<= 8;
        let ok_or = iter.next().ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, Err::EOF))?.1;
        len |= ok_or as u32;
    }
    let mut chunk = [0; 4];
    for i in 0..4 {
        let ok_or = iter.next().ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, Err::EOF))?.1;
        chunk[i] = ok_or;
    }
    let chunk = ChunkId(chunk);
    if len > (i32::MAX as u32) {
        // writeln!(stdinfo, "too large chunk: {}, at byte: {}, len: {}", chunk, ix, len)?;
        return Err(io::Error::new(io::ErrorKind::Other, Err::LargeLen))
    }
    if !chunk.is_valid() {
        // writeln!(stdinfo, "invalid chunk: {}, at byte: {}, len: {}", chunk, ix, len)?;
        return Err(io::Error::new(io::ErrorKind::InvalidData, Err::InvalidData))
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

fn parse_png_chunk(iter: &mut impl Iterator<Item = (usize, u8)>, mut stdinfo: impl Write) -> std::io::Result<Chunk> {
    let (len, chunk) = parse_png_chunk_head(iter, &mut stdinfo)?;
    let bytes: Vec<u8> = iter.by_ref().map(|x| x.1).take(len as usize).collect();
    let mut crc: u32 = 0;
    for _ in 0..4 {
        crc <<= 8;
        let ok_or = iter.next().ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, Err::EOF))?.1;
        crc |= ok_or as u32;
    }
    Ok(Chunk { id: chunk, data: bytes, crc })
}

