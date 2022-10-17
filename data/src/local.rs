use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::chemicals::*;
use serde_json;

static PATH: &str = "data/data.json";

pub fn serialize(compounds: &Data) {
    let data = serde_json::to_string(compounds).unwrap();

    let mut file = File::create(PATH).expect("Error while creating file");

    write!(file, "{}", data).expect("Failed to write to file");
}

pub fn deserialize() -> Vec<Reaction> {
    let file = fs::read_to_string(PATH).expect("cannot read file");
    match serde_json::from_str(file.as_str()) {
        Ok(result) => return result,
        Err(e) => panic!("Failed to deserialize data: {}", e),
    }
}

pub fn data_exists() -> bool {
    let cur_path = Path::new(PATH);
    cur_path.exists()
}
