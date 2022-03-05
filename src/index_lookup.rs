use std::{
    io::{Read, Seek, SeekFrom},
    mem::size_of,
};

use crate::header::Header;

pub struct RbfIndexLookup;

impl RbfIndexLookup {
    pub fn load<T: Read>(reader: &mut T, header: &Header) -> Vec<u32> {
        let number_of_bytes = header.index_count as usize * size_of::<u32>();
        let mut buffer = Vec::with_capacity(number_of_bytes as usize);

        reader.take(number_of_bytes as u64).read_to_end(&mut buffer);

        buffer
            .chunks_exact(4)
            .into_iter()
            .map(|a| u32::from_le_bytes([a[0], a[1], a[2], a[3]]))
            .collect()
    }
}
