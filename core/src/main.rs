use data::chemicals::*;
use data::fetch::update;
use data::local::{deserialize, serialize};
use data::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

    let update_result = update();

    let path = match update_result {
        Ok(s) => s,
        Err(e) => panic!("Update function failed: {}", e),
    };

    let compounds = parser::parse(path);

    println!("There are {} compounds.", compounds.len());

    let data = Data {
        compounds: compounds,
    };

    serialize(&data);
    let compounds = deserialize();

    for c in &compounds {
        println!("{:?}", c)
    }

    println!("There are {} compounds.", compounds.len());
}
