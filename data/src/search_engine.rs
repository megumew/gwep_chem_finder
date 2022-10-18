use std::collections::HashMap;
use crate::chemicals::Reaction;

pub fn generate_search_keys(mut map: HashMap<String, Vec<String>>, reaction: Reaction) -> HashMap<String, Vec<String>> {
    let internal_name = reaction.get_internal_name();
    let result = reaction.get_result().to_lowercase();
    // let name = reaction.get_name().to_lowercase();
    let mut all_keywords: Vec<String> = Vec::new();
    all_keywords.append(&mut string_permutations(internal_name.clone()));
    all_keywords.append(&mut string_permutations(result));
    // all_keywords.append(&mut string_permutations(name));

    for keyword in all_keywords {
        map = insert_keyword(map, keyword, &internal_name);
    }

    map
}

pub fn insert_keyword(mut map: HashMap<String, Vec<String>>, word: String, internal_name: &String) -> HashMap<String, Vec<String>> {
    for k in 0..word.len() {
        match map.get(&word[0..k + 1].to_string()) {
            Some(array) => {
                if array.contains(internal_name) {
                    continue
                } else {
                    map
                        .entry(word[0..k + 1].to_string())
                        .or_default()
                        .push(internal_name.to_string());
                }
            },
            None =>  {
                map
                    .entry(word[0..k + 1].to_string())
                    .or_default()
                    .push(internal_name.to_string());
            }
        }
    }
    map
}

pub fn string_permutations(string: String) -> Vec<String> {
    let mut permmutations: Vec<String> = Vec::new();
    permmutations.push(string.clone());
    if string.clone().contains("_") {
        permmutations.push(string.replace("_", ""));
        permmutations.push(string.replace("_", " "));
        for word in string.split("_") {
            permmutations.push(word.to_string());
        }
    }
    if string.clone().contains(" ") {
        permmutations.push(string.replace(" ", ""));
        permmutations.push(string.replace(" ", "_"));
        for word in string.split(" ") {
            permmutations.push(word.to_string());
        }
    }
    permmutations
}