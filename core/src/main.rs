use std::collections::HashMap;

use data::chemicals::*;
use data::fetch::update;
use data::local::{data_exists, deserialize, serialize};
use data::parser;
extern crate pest;
extern crate pest_derive;

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

    for base in &BASES{
        println!("{}", base.get_id());
    }

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
            compounds: compounds,
        };

        serialize(&data);
    }

    let compounds = deserialize();

    // Hashmap for reagent lookup needs to be constructed after this point

    let mut compound_map: HashMap<String, Compound> = HashMap::with_capacity(compounds.len());

    for c in compounds {
        compound_map.insert(c.get_id(), c);
    }

    // recreate compounds vec after moving references into a lookup hashmap
    let compounds = deserialize();

    let mut compound_trees:Box<HashMap<String, ChemTree>> = Box::new(HashMap::with_capacity(compounds.len()));
    for c in compounds{
        let id = c.get_id();
        let node = ChemTreeNode::new(c.get_result_amount(), Chemical::Compound(c), None);
        println!("{}", node.get_id());
        let mut chem_tree = ChemTree::new(node);
        chem_tree.populate(&compound_map);
        compound_trees.insert(id, chem_tree);
    }

    // compound_trees.get("styptic_powder").unwrap().print_dispenser_format();
    // compound_trees.get("ephedrine").unwrap().print_dispenser_format();
    // compound_trees.get("thermite").unwrap().print_dispenser_format();

    for c in compound_trees.iter(){
        c.1.print_dispenser_format();
    }

}
