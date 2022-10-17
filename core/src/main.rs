use std::collections::HashMap;
use std::env;
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
    println!("Available Bases: {:?}", BASES);

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

    for r in result_map{
        println!("{:?}", r);
    }

    for c in &compounds {
        compound_map.insert(c.get_internal_name(), c.clone());
    }

    let mut compound_trees:Box<HashMap<String, ChemTree>> = Box::new(HashMap::with_capacity(compounds.len()));

    for c in compounds{
        let name = c.get_internal_name();
        let node = ChemTreeNode::new(c.get_specific_reaction_result_amount(0), Chemical::Compound(c), None);
        //println!("{}", node.get_id());
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&compound_map);
        compound_trees.insert(name, chem_tree);
    }


    let args: Vec<String> = env::args().collect();

    // Command Line Interface for looking up Compounds
    if args.len() > 1 && args[1] == "cli"{
        loop {
            println!("Enter your input, or type 'quit' to exit");
            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) =>  {
                    if user_input.trim().to_lowercase() == "quit" || user_input.trim().to_lowercase() == "'quit'" {
                        break
                    }
                },
                Err(_) => println!("Error"),
            }
            let response = compound_trees.get(&user_input.trim().to_lowercase());
    
            match response {
                Some(x) =>  { x.print_dispenser_format() },
                None => { 
                    println!("{} is not a valid Compound!", user_input.trim());
                }
            }
        }
    }
    
}
