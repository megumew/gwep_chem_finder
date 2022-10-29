use std::path::Path;

use crate::{chemicals::*, sql::{add_reaction, add_reactions, get_all_reactions}};

pub fn serialize_to_sql(compounds: Vec<Reaction>) {
    add_reactions(add_reaction(compounds));
}

pub fn deserialize_from_sql() -> Vec<Reaction> {
    get_all_reactions().unwrap()
}

pub fn data_exists(path: &String) -> bool {
    let cur_path = Path::new(path);
    cur_path.exists()
}
