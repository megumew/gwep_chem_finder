use std::collections::HashMap;
use std::io;

use clap::Parser;
use cli::cli::start_cli;
use data::chem_tree::ChemTree;
use data::chemicals::*;
use data::fetch::update;
use data::initialize_maps::initialize_compound_tree;
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
    let initialize: (Box<HashMap<String, ChemTree>>, Maps);
    if updated || !data_exists(&data_string) || args.update {
        initialize = initialize_compound_tree(data_string, paths);
    } else {
        initialize = initialize_compound_tree(data_string, None);
    }

    let reaction_trees: Box<HashMap<String, ChemTree>> = initialize.0;
    let maps = initialize.1;

    // Command Line Interface for looking up Compounds
    if args.cli {
        start_cli(&maps, &reaction_trees);
    }
}
