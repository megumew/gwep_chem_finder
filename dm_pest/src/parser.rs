use chem::chemicals::{Compound, Reagent};
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "DM.pest"]
pub struct DMParser;

//will return data structure containing all of the Compounds
pub fn parse(path: String) -> Vec<Compound> {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");

    let file = DMParser::parse(Rule::file, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));

    let mut compound_pairs = vec![];

    for pairs in file {
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pairs.into_inner() {
            match inner_pair.as_rule() {
                Rule::obj => {
                    compound_pairs.push(inner_pair);
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
                        }
                        "id" => {
                            id = {
                                let str = value.into_inner().next().unwrap();
                                let inner_str = str.into_inner().next().unwrap();
                                String::from(inner_str.as_str())
                            }
                        }
                        "result" => {
                            result = String::from(value.as_str());
                            let val = value.into_inner().next();
                            match val {
                                Some(val) => {
                                    let str = val;
                                    let inner_str = str.into_inner().next().unwrap();
                                    result = String::from(inner_str.as_str())
                                }
                                None => {}
                            }
                        }
                        "mix_phrase" => {
                            mix_phrase = String::from(value.as_str());
                            let val = value.into_inner().next();
                            match val {
                                Some(val) => {
                                    let str = val;
                                    let inner_str = str.into_inner().next().unwrap();
                                    mix_phrase = String::from(inner_str.as_str())
                                }
                                None => {}
                            }
                        }
                        "required_reagents" => {
                            let list = value.into_inner();
                            for val in list {
                                let mut entry = val.into_inner();
                                let data = entry.next().unwrap();
                                let mut data_iter = data.into_inner();
                                let chem_data = data_iter.next().unwrap();
                                let chem = chem_data.into_inner().next().unwrap();
                                let num = data_iter.next().unwrap();

                                required_reagents.push(Reagent::new(
                                    String::from(chem.as_str()),
                                    num.as_str().parse::<u32>().unwrap(),
                                ))
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
