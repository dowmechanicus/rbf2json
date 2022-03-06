use std::{
    io::{Error, Read},
    mem::size_of,
};

use serde::{de::value, Serialize};

use crate::header::Header;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct RbfData {
    pub datatype: u16,
    pub key: u16,
    pub value: RbfDataUnion,
}

#[derive(Debug, Serialize)]
pub enum RbfDataUnion {
    Bval(bool),
    Ival(i32),
    Uval(u32), // Index to the next leave in the JSON structure tree
    Fval(f32),
}

struct JsonData {
    key: JsonDataKey,
}

enum JsonDataKey {
    Ival(i32),
    Fval(f32),
    String(String),
}

impl RbfDataUnion {
    pub fn from(bytes: &[u8]) -> Result<Self, ()> {
        let datatype = u16::from_le_bytes([bytes[0], bytes[1]]);

        match datatype {
            0 => Ok(RbfDataUnion::Bval(
                i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) % 2 == 0,
            )),
            1 => Ok(RbfDataUnion::Fval(f32::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            2 => Ok(RbfDataUnion::Ival(i32::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            3 => Ok(RbfDataUnion::Ival(i32::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            4 => Ok(RbfDataUnion::Uval(u32::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            _ => Err(()),
        }
    }
}

impl RbfData {
    pub fn load<T: Read>(reader: &mut T, header: &Header) -> Vec<RbfData> {
        let number_of_bytes = header.data_count as usize * 8;
        let mut buffer: Vec<u8> = Vec::with_capacity(number_of_bytes);

        reader.take(number_of_bytes as u64).read_to_end(&mut buffer);

        buffer
            .chunks_exact(8)
            .into_iter()
            .map(|a| RbfData {
                datatype: u16::from_le_bytes([a[0], a[1]]),
                key: u16::from_le_bytes([a[2], a[3]]),
                value: RbfDataUnion::from(a).unwrap(),
            })
            .collect()
    }
}
