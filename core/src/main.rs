use chem::chemicals;
use data::update::update;
use dm_pest::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", chemicals::BASES);

    update();

    // let result = dm_reader::dm_reader::read_file(String::from("recipes.DM"));
    let compounds = parser::parse(String::from("data/Chemistry-Recipes.dm"));

    println!("There are {} compounds.", compounds.len());

    // for c in compounds {
    //     println!("{:?}", c)
    // }
}
