use std::{io, collections::HashMap};

use data::{search_engine::*, chemicals::{BASES_MAP, BASES}, chem_tree::ChemTree};

pub fn start_cli(maps: &Maps, reaction_trees: &Box<HashMap<String, ChemTree>>) {
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
        let clean = clean_input(user_input.trim().to_lowercase().to_string());

        if clean.is_empty() {
            println!("Please input a chemical to display or a command with '/'")
        } else if !clean.is_empty() && clean.chars().next().unwrap() == '/' {
            let command = &clean[1..clean.len()];
            let words = command.split_ascii_whitespace().collect::<Vec<&str>>();
            match words.first(){
                Some(w) =>{
                    match w.to_lowercase().as_str() {
                        "h" | "help" => println!("Commands:\n/(r)equires - Displays all reactions required by given chem."),
                        "b" | "bases" => println!("Available Bases: {:?}", BASES),
                        "r" | "requires" => {
                            match words.get(1) {
                                Some(w) => { 
                                    let lookup = match BASES_MAP.get(w){
                                        Some(_) => w.to_string(),
                                    
                                        None => fuzzy_search(&w.to_string(), &maps),
                                    };
                                    let uses = maps.uses_map.get(&lookup);
                                    match uses {
                                        Some(r) => {
                                            println!("\"{}\" is required by {:?}", lookup, r)
                                    },
                                        None => println!("\"{}\" is required by nothing.", lookup)
                                    }

                                },
                                None => println!("This command requires an argument!")
                            }
    
                        }
                        _ => println!("Unkown command: {:?}", words)
                    }
                }
                None => println!("Missing command after /")
                
            }
        } else {
            //check if result and reaction are same to prevent ignoring alternate recipes seperately defined
            match maps.search_map.get(&clean) {
                Some(x) => {
                    if x.len() > 1 {
                        let selection = collision_select(x);
                        reaction_trees
                            .get(&selection)
                            .unwrap()
                            .print_dispenser_format();
                    } else {
                        reaction_trees
                            .get(x.first().unwrap())
                            .unwrap()
                            .print_dispenser_format();
                    }
                }
                None => {
                    let direct = reaction_trees.get(&clean);
                    match direct {
                        Some(x) => x.print_dispenser_format(),
                        None => {
                            let fuzzy = fuzzy_search(&clean, &maps);
                            let search_result = reaction_trees.get(&fuzzy);
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
