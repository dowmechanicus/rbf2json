use byteorder::{BigEndian, ByteOrder, LittleEndian};
use serde::Serialize;
use std::io::Read;

const RBF_VERSION: &str = "RBF V0.1";

#[derive(Debug, Serialize)]
pub struct Header {
    pub version: String,
    pub table_offset: u32,
    pub table_count: u32,
    pub index_offset: u32,
    pub index_count: u32,
    pub data_offset: u32,
    pub data_count: u32,
    pub text_offset: u32,
    pub text_length: u32,
}

impl Header {
    pub fn load<T: Read>(reader: &mut T) -> Self {
        let mut load_part = |size| {
            let mut buf = Vec::with_capacity(size);
            let mut part_reader = reader.take(size as u64);

            part_reader.read_to_end(&mut buf).unwrap();

            buf
        };

        Header {
            version: std::str::from_utf8(&load_part(8)).unwrap().to_string(),
            table_offset: LittleEndian::read_u32(&load_part(4)),
            table_count: LittleEndian::read_u32(&load_part(4)),
            index_offset: LittleEndian::read_u32(&load_part(4)),
            index_count: LittleEndian::read_u32(&load_part(4)),
            data_offset: LittleEndian::read_u32(&load_part(4)),
            data_count: LittleEndian::read_u32(&load_part(4)),
            text_offset: LittleEndian::read_u32(&load_part(4)),
            text_length: LittleEndian::read_u32(&load_part(4)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::header::RBF_VERSION;

    use super::Header;

    #[test]
    fn header_is_correct() {
        let file = File::open("test.rbf").unwrap();
        let mut reader = BufReader::new(file);

        let header = Header::load(&mut reader);

        assert_eq!(header.version, RBF_VERSION);
    }
}
