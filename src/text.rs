use std::{io::Read, mem::size_of};

use crate::header::Header;

pub struct RbfStrings;

impl RbfStrings {
    pub fn load<T: Read>(reader: &mut T, header: &Header) -> String {
        let number_of_bytes = header.text_length as usize * size_of::<char>();
        let mut buffer = Vec::with_capacity(number_of_bytes);

        reader.take(number_of_bytes as u64).read_to_end(&mut buffer);

        std::str::from_utf8(&buffer).unwrap().to_string()
    }
}
