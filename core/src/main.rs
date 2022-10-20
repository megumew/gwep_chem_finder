use std::collections::HashMap;
use std::io;

use clap::Parser;
use data::initialize_maps::initialize_compound_tree;
use data::chem_tree::ChemTree;
use data::chemicals::*;
use data::fetch::update;
use data::local::data_exists;
use data::search_engine::*;
extern crate pest;
extern crate pest_derive;

/// Gwep Chem Finder
#[derive(clap::Parser)]
#[command()]
struct Args {
    ///Forces the program to update
    #[arg(short, long)]
    update: bool,
    ///Runs the program in CLI mode
    #[arg(short, long)]
    cli: bool,
}

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

    let args = Args::parse();

    let update_result = update();

    let updated;
    let paths = match update_result {
        (s, b) => {
            updated = b;
            Some(s)
        }
    };
    let data_string = "data/data.json".to_string();
    let initialize:(Box<HashMap<String, ChemTree>>, Maps);
    if updated || !data_exists(&data_string) || args.update {
        initialize = initialize_compound_tree(data_string, paths);
    } else {
        initialize = initialize_compound_tree(data_string, None);
    }

    let compound_trees: Box<HashMap<String, ChemTree>> = initialize.0;
    let maps = initialize.1;

    // Command Line Interface for looking up Compounds
    if args.cli {
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

            if clean.is_empty(){
                println!("Please input a chemical to display or a command with '/'")
            }
            else if !clean.is_empty() && clean.chars().next().unwrap() == '/' {
                let command = &clean[1..clean.len()];
                let words = command.split_ascii_whitespace().collect::<Vec<&str>>();
                match words.first(){
                    Some(w) =>{
                        match w.to_lowercase().as_str() {
                            "h" | "help" => println!("Commands:\n/(r)equires - Displays all reactions required by given chem."),
                            "r" | "requires" => {
                                match words.get(1) {
                                    Some(w) => println!("\"{}\" is required by TEST", w),
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
                                let fuzzy = fuzzy_search(&clean, &maps);
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
}
