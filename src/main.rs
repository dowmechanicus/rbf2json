#![allow(dead_code, unused)]
#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};

use data::RbfData;
use header::Header;
use index_lookup::RbfIndexLookup;
use serde::Serialize;
use table::RbfTable;

use crate::text::RbfStrings;

mod data;
mod header;
mod index_lookup;
mod table;
mod text;

#[derive(Debug, Serialize)]
pub struct RbfBufferData<'a> {
    pub header: Header,
    pub table: Vec<RbfTable>,
    pub data: Vec<RbfData>,
    pub index: Vec<u32>,
    pub text: &'a str,
}

fn main() {
    let file = File::open("test.rbf").unwrap();
    let mut reader = BufReader::new(file);

    let header = Header::load(&mut reader);
    let table = RbfTable::load(&mut reader, &header);
    let index = RbfIndexLookup::load(&mut reader, &header);
    let data = RbfData::load(&mut reader, &header);
    let text = RbfStrings::load(&mut reader, &header);

    let buffer_data = RbfBufferData {
        header: header,
        table: table,
        data: data,
        index: index,
        text: &text,
    };

    RbfTable::table2json(&buffer_data, 0);

    println!("{}", serde_json::to_string_pretty(&buffer_data).unwrap());
}
