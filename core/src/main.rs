use std::collections::HashMap;

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

    // Hashmap for reagent lookup needs to be constructed after this point

    let mut compound_map: HashMap<String, Compound> = HashMap::with_capacity(compounds.len());

    for c in compounds {
        compound_map.insert(c.id.clone(), c);
    }

    // recreate compounds vec after moving references into a lookup hashmap
    let compounds = deserialize();

    println!("There are {} compounds.", compounds.len());
}
