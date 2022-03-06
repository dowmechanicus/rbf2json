use std::{io::Read, iter::Sum, mem::size_of};

use byteorder::{BigEndian, ByteOrder};
use serde::Serialize;

use crate::{data::RbfDataUnion, header::Header, RbfBufferData};

use super::array_names::ARRAY_NAMES;

#[derive(Debug, Serialize)]
pub struct RbfTable {
    child_count: u32,
    child_index: u32,
}

impl RbfTable {
    pub fn load<'a, T: Read>(reader: &mut T, header: &'a Header) -> Vec<RbfTable> {
        let number_of_bytes = header.table_count as usize * size_of::<RbfTable>();
        let mut buffer: Vec<u8> = Vec::with_capacity(number_of_bytes);

        reader.take(number_of_bytes as u64).read_to_end(&mut buffer);

        buffer
            .chunks_exact(8)
            .into_iter()
            .map(|a| RbfTable {
                child_count: u32::from_le_bytes([a[0], a[1], a[2], a[3]]),
                child_index: u32::from_le_bytes([a[4], a[5], a[6], a[7]]),
            })
            .collect()
    }

    pub fn table2json(buffer_data: &RbfBufferData, index: u32) -> u32 {
        let table = &buffer_data.table[index as usize];

        match table.child_count {
            0 => 0,
            1 => {
                // if json_is_array(root)
                todo!("rbf_entry2json");
                todo!("json_array_append");
                todo!("json_decref");
                // else
                todo!("rbf_entry2json");
            }
            count => {
                for i in 0..table.child_count {
                    let child_index = buffer_data.index[(table.child_index + i) as usize];
                    println!("child index: {}", child_index);
                    RbfTable::rbf_entry_to_json(buffer_data, child_index);
                }

                count
            }
        }
    }

    fn rbf_entry_to_json(buffer_data: &RbfBufferData, child_index: u32) {
        let data_entry = &buffer_data.data[child_index as usize];
        println!("{:?}", data_entry);

        match data_entry.datatype {
            0 | 1 | 2 | 3 => RbfTable::get_value_from_datatype(&data_entry.value),
            4 => {
                if RbfTable::rbl_table_is_array(key_name) {}
            },
            _ => panic!("datatype does not exist"),
        }
    }

    fn get_value_from_datatype(value: &RbfDataUnion) {
        match value {
            RbfDataUnion::Bval(value) => println!("{}", value),
            RbfDataUnion::Ival(value) => println!("{}", value),
            RbfDataUnion::Uval(value) => println!("{}", value),
            RbfDataUnion::Fval(value) => println!("{}", value),
        }
    }

    fn rbl_table_is_array(key_name: &str) -> bool {
        for i in 0..ARRAY_NAMES.len() {
            if (key_name == ARRAY_NAMES[i]) {
                return true
            }
        }

        false
    } 
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::RbfTable;

    #[test]
    fn struct_has_correct_size_in_bytes() {
        assert_eq!(size_of::<RbfTable>(), 8);
    }
}
