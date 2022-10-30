use crate::chemicals::Reaction;
use std::{collections::HashMap, io};

pub struct Maps {
    pub reaction_map: HashMap<String, Reaction>,
    pub result_map: HashMap<String, Vec<String>>,
    pub search_map: HashMap<String, Vec<String>>,
    pub uses_map: HashMap<String, Vec<String>>,
}

pub fn generate_search_keys(
    mut map: HashMap<String, Vec<String>>,
    reaction: Reaction,
) -> HashMap<String, Vec<String>> {
    let internal_name = reaction.get_internal_name();
    let result = reaction.get_result().to_lowercase();
    let name = reaction.get_name().to_lowercase();
    let mut all_keywords: Vec<String> = Vec::new();
    all_keywords.append(&mut string_permutations(internal_name.clone()));
    all_keywords.append(&mut string_permutations(result));
    all_keywords.append(&mut string_permutations(name));

    for keyword in all_keywords {
        map = insert_keyword(map, keyword, &internal_name);
    }

    map
}

fn insert_keyword(
    mut map: HashMap<String, Vec<String>>,
    word: String,
    internal_name: &String,
) -> HashMap<String, Vec<String>> {
    for k in 0..word.len() {
        let chars = word.chars();
        let string: String = chars.take(k + 1).collect();
        match map.get(&string) {
            Some(array) => {
                if array.contains(internal_name) {
                    continue;
                } else {
                    map.entry(string)
                        .or_default()
                        .push(internal_name.to_string());
                }
            }
            None => {
                map.entry(string)
                    .or_default()
                    .push(internal_name.to_string());
            }
        }
    }
    map
}

fn string_permutations(string: String) -> Vec<String> {
    let mut permmutations: Vec<String> = Vec::new();
    permmutations.push(string.clone());
    // Thankfully there are no instances of ["_" and "-" or " "] being in the same name
    if string.clone().contains("-") || string.clone().contains(" ") {
        // First, pushes all permutations using "-", "", " ", or "_" as fillers between words
        permmutations.push(string.replace("-", ""));
        permmutations.push(string.replace(" ", ""));
        permmutations.push(string.replace(" ", "").replace("-", ""));
        permmutations.push(string.replace(" ", "-"));
        let only_spaces = string.replace("-", " ");
        permmutations.push(only_spaces.clone());
        // Uses the last part with " " between words to make accessing each individual word easier
        let mut only_words = only_spaces.split_whitespace(); // split_whitespace works with multiple spaces in a row
        only_words.next();
        // Loop pushes each word individually [Very, High, Fructose, Corn, Syrup], and sets up for inner loop
        loop {
            let word = only_words.next();
            if word == None {
                return permmutations;
            }
            let word_to_string = word.unwrap().to_string();
            permmutations.push(word_to_string.clone());
            let space_clone = only_spaces.clone();
            let mut clone = space_clone.split_whitespace();
            let mut more_words: String = "".to_string();
            // Inner loop gets all word order permutations, for example [Very Fructose, High Corn, Fructose Syrup]
            loop {
                let next = clone.next();
                if next == None {
                    break;
                }
                more_words.push_str(next.unwrap());
                if &word_to_string != &next.unwrap() {
                    permmutations.push(format!("{}{}", &word_to_string, &next.unwrap()));
                    permmutations.push(format!("{} {}", &word_to_string, &next.unwrap()));
                    permmutations.push(format!("{}-{}", &word_to_string, &next.unwrap()));
                    permmutations.push(format!("{}_{}", &word_to_string, &next.unwrap()));
                }
                if &word_to_string != &more_words {
                    permmutations.push(more_words.clone());
                    permmutations.push(format!("{}{}", &word_to_string, &more_words));
                    permmutations.push(format!("{} {}", &word_to_string, &more_words));
                    permmutations.push(format!("{}-{}", &word_to_string, &more_words));
                    permmutations.push(format!("{}_{}", &word_to_string, &more_words));
                }
            }
        }
    }
    if string.clone().contains("_") {
        permmutations.push(string.replace("_", ""));
        permmutations.push(string.replace("_", " "));
        permmutations.push(string.replace("_", "-"));
        let mut no_underscores = string.split("_");
        no_underscores.next(); // The first word is covered by other permutations
        loop {
            let word = no_underscores.next();
            if word == None {
                return permmutations;
            }
            permmutations.push(word.unwrap().to_string());
        }
    }
    permmutations
}

//Returns a string for the compound trees
pub fn fuzzy_search(input: &String, maps: &Maps) -> String {
    let mut best_score: (i32, String) = (i32::MAX, String::new());
    for x in &maps.search_map {
        let diff = score_diff(&x.0, input);

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

    let result = maps.search_map.get(&best_score.1).unwrap();
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
            '-' => ' ',
            _ => x,
        })
        .collect();

    let input_c: String = input
        .chars()
        .map(|x| match x {
            '_' => ' ',
            '-' => ' ',
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

pub fn collision_select(result: &Vec<String>) -> String {
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
                            "Please enter only a valid number! (range: {}-{})",
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
