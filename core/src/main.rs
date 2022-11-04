use clap::Parser;
use cli::cli::start_cli;
use data::chemicals::BASES;
use data::initialize::initialize;
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

    initialize(args.update);

    // Command Line Interface for looking up Compounds
    if args.cli {
        start_cli();
    }
}