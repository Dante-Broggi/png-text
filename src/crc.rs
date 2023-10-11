
/* Table of CRCs of all 8-bit messages. */
// const CRC_TABLE: [u32; 256] = make_crc_table();

/** Make the table for a fast CRC. */
fn make_crc_table() -> [u32; 256] {
    let mut table = [0; 256];
    for n in 0..256 {
        let mut c = n as u32;
        for _ in 0..8 {
            if (c & 1) == 1 {
                c = 0xedb88320 ^ ((c >> 1) & 0x7FFFFFFF);
            } else {
                c = (c >> 1) & 0x7FFFFFFF;
            }
        }
        table[n] = c;
    }
    table
}

/** Update a running CRC with the bytes buf[0..len-1]--the CRC
   should be initialized to all 1's, and the transmitted value
   is the 1's complement of the final running CRC (see the
   crc() routine below).
*/
fn update_crc(crc: u32, buf: &[u8]) -> u32 {
    let mut c = crc;
    for b in buf {
        c = make_crc_table()[((c ^ (*b as u32)) & 0xff) as usize] ^ ((c >> 8) & 0xFFFFFF);
    }
    c
}

pub fn crc(head: crate::ChunkId, buf: &[u8]) -> u32 {
    let c = update_crc(0xffffffff, &head.0);
    update_crc(c, buf) ^ 0xffffffff
}

// pub fn crc_raw(buf: &[u8]) -> u32 {
//     update_crc(0xffffffff, buf) ^ 0xffffffff
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_iend() {
        let iend = crate::Chunk {
            id: crate::ChunkId::IEND,
            data: vec![],
            crc: u32::from_be_bytes([0xae, 0x42, 0x60, 0x82]),
        };
        assert_eq!(crc(iend.id, &iend.data), iend.crc);
        assert!(iend.is_valid_spec());
    }
}
