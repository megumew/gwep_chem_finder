use std::collections::HashMap;
use std::io;

use data::chem_tree::{ChemTree, ChemTreeNode};
use data::chemicals::*;
use data::fetch::update;
use data::local::{data_exists, deserialize, serialize};
use data::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");

    // for base in &BASES{
    //     println!("{}", base.get_id());
    // }

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
            compounds,
        };

        serialize(&data);
    }

    let compounds = deserialize();

    // Hashmap for reagent lookup needs to be constructed after this point

    let mut compound_map: HashMap<String, Compound> = HashMap::with_capacity(compounds.len());
    let mut result_map: HashMap<String, Vec<String>> = HashMap::with_capacity(compounds.len());

    // registers all possible results with their respective ID
    for c in &compounds{
        if !c.get_result().is_empty(){
            result_map.entry(c.get_result()).or_default().push(c.get_internal_name()); 
        }
    }

    for c in &compounds {
        compound_map.insert(c.get_internal_name(), c.clone());
    }

    let mut compound_trees:Box<HashMap<String, ChemTree>> = Box::new(HashMap::with_capacity(compounds.len()));

    for c in compounds.clone() {
        let name = c.get_internal_name();
        let node = ChemTreeNode::new(c.get_specific_reaction_result_amount(0), Chemical::Compound(c), None);
        //println!("{}", node.get_id());
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&compound_map); //issue here
        compound_trees.insert(name, chem_tree);
    }

    // Command Line Interface for looking up Compounds
    loop {
        println!("Enter your input, command, or type '-help' to show all commands");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) =>  {},
            Err(_) => println!("Error"),
        }
        let trimmed = user_input.trim();
        let lowercase = trimmed.to_lowercase();
        if lowercase == "-quit" {
            println!("Goodbye :)");
            break
        }
        else if lowercase == "-help" || lowercase == "help" {
            println!("-help: Prints out this page");
            println!("-quit: Quits the program");
            println!("-list: Lists out all available Compounds for lookup");
            println!("-bases: Lists out all Bases");
            println!("To look up a specific Compound, simply input its name in");
        }
        else if lowercase == "-list" {
            println!("There are {} compounds.", compounds.len());
            for r in &result_map {
                println!("{:?}", r);
            }
        }
        else if lowercase == "-bases" {
            println!("Available Bases: {:?}", BASES);
        }
        else {
            let response = compound_trees.get(&lowercase);
            match response {
                Some(x) =>  { x.print_dispenser_format() },
                None => {
                    println!("{} is not a valid Compound!", trimmed);
                }
            }
        }
    }
}
