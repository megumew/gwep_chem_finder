use std::{io, collections::HashMap};

use data::{search_engine::*, chemicals::{BASES_MAP, BASES}, chem_tree::ChemTree};

use crate::print::print_dispenser_format;

pub fn start_cli(maps: &Maps, reaction_trees: &Box<HashMap<String, ChemTree>>) {
    'cli: loop {
        println!("Enter your input, or type '/help' to see commands");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
        let clean = clean_input(user_input.trim().to_lowercase().to_string());

        if clean.is_empty() {
            println!("Please input a chemical to display or a command with '/'")
        } else if !clean.is_empty() && clean.chars().next().unwrap() == '/' {
            let command = &clean[1..clean.len()];
            let words: Vec<&str> = command.split_ascii_whitespace().collect();
            match words.first(){
                Some(w) =>{
                    match w.to_lowercase().as_str() {
                        "q" | "quit" => { break 'cli }
                        "h" | "help" => { 
                            println!("\nCommands:\n\n/(r)equires - Displays all reactions required by given chem.");
                            println!("/(b)ases - Displays all bases used in-game.");  
                            println!("/(h)elp - Displays this help page.");  
                            println!("/(q)uit - Quits the program."); 
                        }
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
                        print_dispenser_format(reaction_trees.get(&selection).unwrap().clone());
                    } else {
                        print_dispenser_format(reaction_trees.get(&x[0]).unwrap().clone());
                    }
                }
                None => {
                    let direct = reaction_trees.get(&clean);
                    match direct {
                        Some(x) => print_dispenser_format(x.clone()),
                        None => {
                            let fuzzy = fuzzy_search(&clean, &maps);
                            let search_result = reaction_trees.get(&fuzzy);
                            match search_result {
                                Some(x) => print_dispenser_format(x.clone()),
                                None => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
