use data::chemicals::*;
use data::fetch::update;
use data::local::{data_exists, deserialize, serialize};
use data::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

    // Find a way to track if the script exists
    let update_result = update();

    let updated;
    let path = match update_result {
        (s, b) => {
            updated = b;
            s
        }
    };

    // Consider adding a force update bool based off launch parameters or if an error occurs
    if updated || !data_exists() {
        let compounds = parser::parse(path);

        println!("There are {} compounds.", compounds.len());

        let data = Data {
            compounds: compounds,
        };

        serialize(&data);
    }

    let compounds = deserialize();

    for c in &compounds {
        println!("{:?}", c.chem_dispenser_format())
    }

    println!("There are {} compounds.", compounds.len());
}
