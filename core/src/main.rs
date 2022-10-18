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

    // registers all possible results with their respective internal names
    for reaction in &reactions {
        if !reaction.get_result().is_empty() {
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
            match result_map.get(&clean) {
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
                            let fuzzy = fuzzy_search(&clean, &result_map);
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

//Returns a string for the compound trees
fn fuzzy_search(input: &String, data: &HashMap<String, Vec<String>>) -> String {
    let mut best_score: (i32, String) = (i32::MAX, String::new());
    for x in data {
        let diff = score_diff(x.0, input);

        if diff.0 == 0 {
            best_score = diff;
            break;
        }

        if diff.0 < best_score.0 {
            best_score = diff;
        }
    }
    println!(
        "Closest Match: {} with a score of {}",
        best_score.1, best_score.0
    );

    let result = data.get(&best_score.1).unwrap();
    if result.len() > 1 {
        best_score.1 = collision_select(result);
    } else {
        best_score.1 = result.get(0).unwrap().to_string()
    }

    best_score.1
}

fn score_diff(searched: &String, input: &String) -> (i32, String) {
    // Use these iterator functions to clean the input to match
    let searched_c: String = searched
        .chars()
        .map(|x| match x {
            '_' => ' ',
            _ => x,
        })
        .collect();

    let input_c: String = input
        .chars()
        .map(|x| match x {
            '_' => ' ',
            _ => x,
        })
        .collect();

    let mut total_diff = 0;
    let longer: String;
    let shorter: String;

    if searched.len() > input.len() {
        longer = searched_c;
        shorter = input_c;
    } else {
        shorter = searched_c;
        longer = input_c;
    }

    let mut s_chars = shorter.chars();

    for c1 in longer.chars() {
        match s_chars.next() {
            Some(c2) => {
                let diff = c1 as i32 - c2 as i32;
                total_diff += diff.abs();
            }
            None => {
                total_diff += 26;
            }
        }
    }
    (total_diff, searched.to_string())
}

fn clean_input(input: String) -> String {
    let words: Vec<_> = input.split_whitespace().collect();
    words.join(" ")
}

fn collision_select(result: &Vec<String>) -> String {
    println!(
        "Found {} possible options. Please select one to continue.",
        result.len()
    );
    for (i, r) in result.iter().enumerate() {
        println!("{}. {}", i + 1, r);
    }

    let mut selection = String::new();
    let mut valid = false;
    while !valid {
        let mut i_num = String::new();
        match io::stdin().read_line(&mut i_num) {
            Ok(_) => match i_num.trim().parse::<usize>() {
                Ok(mut i) => {
                    i -= 1;
                    if i < result.len() {
                        selection = result.get(i).unwrap().to_string();
                        println!("Selecting {} ({})", i + 1, selection);
                        valid = true;
                    } else {
                        println!(
                            "Please enter only a valid number! (range {}-{})",
                            1,
                            result.len()
                        );
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            },
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    selection
}
