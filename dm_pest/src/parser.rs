use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "DM.pest"]
pub struct DMParser;

pub fn read_file(path: String) {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");

    let file = DMParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
}
