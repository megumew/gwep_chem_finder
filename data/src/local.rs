use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::chemicals::*;
use serde_json::{Result, Value};

static PATH: &str = "data/data.json";

pub fn serialize(compounds: &Data) {
    let data = serde_json::to_string(compounds).unwrap();

    let mut file = File::create(PATH).expect("Error while creating file");

    write!(file, "{}", data).expect("Failed to write to file");
}

pub fn deserialize() -> Vec<Compound> {
    let file = fs::read_to_string(PATH).expect("cannot read file");
    match serde_json::from_str(file.as_str()) {
        Ok(result) => return result,
        Err(e) => panic!("Failed to deserialize data: {}", e),
    }
}
