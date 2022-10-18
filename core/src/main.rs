use std::collections::HashMap;
use std::env;
use std::io;

use data::chem_tree::{ChemTree, ChemTreeNode};
use data::chemicals::*;
use data::search_engine::*;
use data::fetch::update;
use data::local::{data_exists, deserialize, serialize};
use data::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

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
        let reactions = parser::parse(path);

        println!("There are {} compounds.", reactions.len());

        let data = Data {
            compounds: reactions,
        };

        serialize(&data);
    }

    let reactions = deserialize();

    //This is a map of all the rection names
    let mut reaction_map: HashMap<String, Reaction> = HashMap::with_capacity(reactions.len());
    let mut result_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    let mut search_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    // registers all possible results with their respective internal names
    for reaction in &reactions {
        if !reaction.get_result().is_empty() {
            search_map = generate_search_keys(search_map, reaction.clone());
            result_map
                .entry(reaction.get_result())
                .or_default()
                .push(reaction.get_internal_name());
        }
    }

    for r in &result_map {
        if r.1.len() > 1 {
            println!("{:?}", r);
        }
    }

    for reaction in &reactions {
        reaction_map.insert(reaction.get_internal_name(), reaction.clone());
    }

    let mut compound_trees: Box<HashMap<String, ChemTree>> =
        Box::new(HashMap::with_capacity(reactions.len()));

    for reaction in reactions {
        let name = reaction.get_internal_name();
        let node = ChemTreeNode::new(
            reaction.get_specific_recipe_result_amount(0),
            Chemical::Compound(reaction),
            None,
        );
        //println!("{}", node.get_id());
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&reaction_map);
        compound_trees.insert(name, chem_tree);
    }

    // Command Line Interface for looking up Compounds
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "cli" {
        loop {
            println!("Enter your input, or type 'quit' to exit");
            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {
                    if user_input.trim().to_lowercase() == "quit"
                        || user_input.trim().to_lowercase() == "'quit'"
                    {
                        break;
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
            let clean = clean_input(user_input.trim().to_string());

            //check if result and reaction are same to prevent ignoring alternate recipes seperately defined
            match search_map.get(&clean) {
                Some(x) => {
                    if x.len() > 1 {
                        let selection = collision_select(x);
                        compound_trees
                            .get(&selection)
                            .unwrap()
                            .print_dispenser_format();
                    } else {
                        compound_trees
                            .get(x.first().unwrap())
                            .unwrap()
                            .print_dispenser_format();
                    }
                }
                None => {
                    let direct = compound_trees.get(&clean);
                    match direct {
                        Some(x) => x.print_dispenser_format(),
                        None => {
                            let fuzzy = fuzzy_search(&clean, &search_map);
                            let search_result = compound_trees.get(&fuzzy);
                            match search_result {
                                Some(x) => x.print_dispenser_format(),
                                None => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

