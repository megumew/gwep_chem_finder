use data::chemicals::{Compound, Reagent};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "DM.pest"]
pub struct DMParser;

//will return data structure containing all of the Compounds
pub fn parse(path: String) -> Vec<Compound> {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");

    // let pairs = DMParser::parse(Rule::file, &unparsed_file)
    //     .expect("unsuccessful parse") // unwrap the parse result
    //     .next()
    //     .unwrap(); // get and unwrap the `file` rule; never fails

    let file = DMParser::parse(Rule::file, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));

    let mut compound_pairs = vec![];

    for pairs in file {
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pairs.into_inner() {
            match inner_pair.as_rule() {
                Rule::obj => {
                    //println!("{}", inner_pair.as_str());
                    compound_pairs.push(inner_pair);
                    // let mut internal_name: String = String::new();
                    // let mut name: String = String::new();
                    // let mut id: String = String::new();
                    // let mut result: String = String::new();
                    // let mut mix_phrase: String = String::new();
                    // let mut required_reagents: Vec<Reagent> = Vec::new();
                    // let mut result_amount: u8 = 0;
                    // let mut hidden: Option<bool> = None;
                    // //println!("Compound:  {}", inner_pair.as_str());
                    // for value in inner_pair.into_inner() {
                    //     match value.as_rule() {
                    //         Rule::identifier => internal_name = String::from(value.as_str()),
                    //         Rule::field => {
                    //             println!("{}", value.as_str());
                    //             compounds.push(value);
                    //         }
                    //         _ => {}
                    //     }
                    // }

                    // println!(
                    //     "Compound: {}\nname: {}\nid: {}\nresult: {}\nmixphrase: {}\nrequired_reagents{:?}\nresult_amount: {}\nhidden: {:?}",
                    //     internal_name,
                    //     name,
                    //     id,
                    //     result,
                    //     mix_phrase,
                    //     required_reagents,
                    //     result_amount,
                    //     &hidden
                    // );
                }
                _ => {}
            };
        }
    }
    to_struct(compound_pairs)
}

fn to_struct(pairs: Vec<pest::iterators::Pair<Rule>>) -> Vec<Compound> {
    let mut compounds: Vec<Compound> = Vec::new();

    for pair in pairs {
        let mut internal_name: String = String::new();
        let mut name: String = String::new();
        let mut id: String = String::new();
        let mut result: String = String::new();
        let mut mix_phrase: String = String::new();
        let mut required_reagents: Vec<Reagent> = Vec::new();
        let mut result_amount: f32 = 0.0;
        let mut hidden: Option<bool> = None;

        for line in pair.into_inner() {
            match line.as_rule() {
                Rule::identifier => internal_name = String::from(line.as_str()),
                Rule::field => {
                    let mut pair = line.into_inner();
                    let field = pair.next().unwrap();
                    let mut data = field.into_inner();
                    //println!("{}", data.as_str());
                    let identifier = data.next().unwrap();
                    let value = data.next().unwrap();
                    //println!("{} {}", identifier.as_str(), value.as_str());
                    match identifier.as_str() {
                        "name" => {
                            let str = value.into_inner().next().unwrap();
                            let inner_str = str.into_inner().next().unwrap();
                            name = String::from(inner_str.as_str());
                            println!("name: {}", &name);
                        }
                        "id" => {
                            id = {
                                let str = value.into_inner().next().unwrap();
                                let inner_str = str.into_inner().next().unwrap();
                                String::from(inner_str.as_str())
                            }
                        }
                        "result" => result = String::from(value.as_str()),
                        "mix_phrase" => mix_phrase = String::from(value.as_str()),
                        "required_reagents" => {
                            let list = value.into_inner();
                            println!("{}", list.as_str());
                            for val in list {
                                println!("{}", val.as_str())
                            }
                        }
                        "result_amount" => result_amount = value.as_str().parse::<f32>().unwrap(),
                        "hidden" => hidden = Some(true),
                        _ => {}
                    }
                }
                Rule::definition => {
                    println!("Nested def not implemented yet!\nData: {}", line.as_str())
                }
                _ => println!("{:?}", line.as_rule()),
            }
        }

        compounds.push(Compound::new(
            internal_name,
            name,
            id,
            result,
            mix_phrase,
            required_reagents,
            result_amount,
            hidden,
        ))
    }
    compounds
}
