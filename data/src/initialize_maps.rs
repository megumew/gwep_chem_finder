use crate::{
    chem_tree::{ChemTree, ChemTreeNode},
    chemicals::{Chemical, Data, Reaction},
    local::{deserialize, serialize},
    parser,
    search_engine::{generate_search_keys, Maps},
};
use std::collections::HashMap;

pub fn initialize_compound_tree(
    serialize_path: String,
    optional_path: Option<String>,
) -> (Box<HashMap<String, ChemTree>>, Maps) {
    match optional_path {
        Some(path) => {
            let reactions = parser::parse(path);
            println!("There are {} compounds.", reactions.len());
            let data = Data {
                compounds: reactions,
            };
            serialize(&data, serialize_path.clone());
        }
        None => {}
    }
    let reactions = deserialize(serialize_path);
    let mut reaction_map: HashMap<String, Reaction> = HashMap::with_capacity(reactions.len());
    let mut result_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    let mut search_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    let mut uses_map: HashMap<String, Vec<String>> = HashMap::with_capacity(reactions.len());
    // registers all possible results with their respective internal names
    for reaction in &reactions {
        if !reaction.get_result().is_empty() {
            search_map = generate_search_keys(search_map, reaction.clone());
            result_map
                .entry(reaction.get_result())
                .or_default()
                .push(reaction.get_internal_name());
        }
        reaction_map.insert(reaction.get_internal_name(), reaction.clone());

        for recipe in reaction.get_all_recipes() {
            for reagent in recipe {
                let name = reagent.name.clone();
                match uses_map.get(&name) {
                    None => {
                        uses_map
                            .entry(name)
                            .or_default()
                            .push(reaction.get_internal_name());
                    }
                    Some(result) => {
                        if !result.contains(&name) {
                            uses_map
                                .entry(name)
                                .or_default()
                                .push(reaction.get_internal_name());
                        }
                    }
                }
            }
        }
    }

    let maps = Maps {
        reaction_map,
        result_map,
        search_map,
        uses_map,
    };

    let mut compound_trees: Box<HashMap<String, ChemTree>> =
        Box::new(HashMap::with_capacity(reactions.len()));

    for reaction in reactions {
        let name = reaction.get_internal_name();
        let node = ChemTreeNode::new(
            reaction.get_specific_recipe_result_amount(0),
            Chemical::Compound(reaction),
            None,
        );
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&maps);
        compound_trees.insert(name, chem_tree);
    }
    (compound_trees, maps)
}
