use data::chemicals::{Compound, Reagent};
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "DM.pest"]
pub struct DMParser;

pub fn read_file(path: String) {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");

    // let pairs = DMParser::parse(Rule::file, &unparsed_file)
    //     .expect("unsuccessful parse") // unwrap the parse result
    //     .next()
    //     .unwrap(); // get and unwrap the `file` rule; never fails

    let pairs = DMParser::parse(Rule::file, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::obj => {
                    let internal_name: String;
                    let name: String;
                    let id: String;
                    let result: String;
                    let mix_phrase: String;
                    let required_reagents: Vec<Reagent>;
                    let result_amount: u8;
                    let hidden: Option<bool>;
                    //println!("Compound:  {}", inner_pair.as_str());
                    for field in inner_pair.into_inner() {
                        println!("field: {}", field.as_str())
                    }
                    println!();
                }
                //Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => {}
            };
        }
    }
}
