use chem::chemicals;
use data::update::update;
use dm_pest::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", chemicals::BASES);

    let update_result = update();

    let path = match update_result {
        Ok(s) => s,
        Err(e) => panic!("Update function failed: {}", e),
    };

    // let result = dm_reader::dm_reader::read_file(String::from("recipes.DM"));
    let compounds = parser::parse(path);

    println!("There are {} compounds.", compounds.len());

    // for c in compounds {
    //     println!("{:?}", c)
    // }
}
