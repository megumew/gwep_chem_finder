use std::fmt;

use data::chemicals;
use dm_pest::parser;
// use dm_reader;
extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", chemicals::BASES);

    // let result = dm_reader::dm_reader::read_file(String::from("recipes.DM"));
    let file = parser::read_file(String::from("recipes.DM"));
}
