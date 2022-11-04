use data::{
    chemicals::BASES,
    search_engine::*, sql::fetch_reaction,
};

use crate::print::print_dispenser_format;

pub fn start_cli() {
    let mut toggle = false;

    'cli: loop {
        println!("Enter your input, or type '/help' to see commands");
        let mut user_input = String::new();
        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
        let clean = user_input.to_lowercase().split_whitespace().collect::<Vec<&str>>().join(" ");

        if clean.is_empty() {
            println!("Please input a chemical to display or a command with '/'")
        } else if !clean.is_empty() && clean.chars().next().unwrap() == '/' {
            let command = &clean[1..clean.len()];
            let words: Vec<&str> = command.split_ascii_whitespace().collect();
            match words.first() {
                Some(w) => match w.to_lowercase().as_str() {
                    "q" | "quit" => break 'cli,
                    "t" | "toggle" => {
                        if toggle == true {
                            println!("Showing recipes without a %");
                            toggle = false
                        } else {
                            println!("Showing recipes as a %");
                            toggle = true
                        }
                    }
                    "h" | "help" => print_help(),
                    "b" | "bases" => println!("Available Bases: {:?}", BASES),
                    "r" | "requires" => match words.get(1) {
                        Some(_) => {
                            requires(words[1..words.len()].join(" ").as_str());
                        }
                        None => println!("This command requires an argument!"),
                    },
                    _ => println!("Unknown command: {:?}", words),
                },
                None => println!("Missing command after /"),
            }
        } else {
            let search = reaction_search(&clean);
            match search {
                Ok(s) => {
                    let fuzzy =  if s.len() > 1 {
                        collision_select(&s)
                    } else {
                        s[0].to_string()
                    };
                    let search_result = fetch_reaction(fuzzy);
                    print_dispenser_format(search_result, toggle);
                }
                Err(e) => {println!("Error: {}", e)}
            }
        }
    }
}

fn requires(w: &str) {
    let lookup = if BASES.contains(&w) {
        w.to_string()
    } else {
        let reagent_search = reagent_search(&w.to_string()).unwrap();
        if reagent_search.len() > 1 {
            collision_select(&reagent_search)
        } else {
            reagent_search[0].to_string()
        }
    };
    let uses = reagent_uses(lookup.clone());
    match uses {
        Ok(list) => {
            if list.len() > 0 {
                println!("\"{}\" is required by {:?}", lookup, list);
            } else {
                println!("\"{}\" is required by nothing.", lookup);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn print_help() {
    println!("\nCommands:\n---------");
    println!("/(r)equires\n\t\tDisplays all reactions required by given chem.");
    println!("/(b)ases\n\t\tDisplays all bases used in-game.");
    println!("/(h)elp\n\t\tDisplays this help page.");
    println!("/(q)uit\n\t\tQuits the program.");
    println!("---------");
}
