use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// static BASES: [Base; 30] = [
//     Base { id: "aluminium" },
//     Base { id: "barium" },
//     Base { id: "bromine" },
//     Base { id: "calcium" },
//     Base { id: "carbon" },
//     Base { id: "chlorine" },
//     Base { id: "chromium" },
//     Base { id: "copper" },
//     Base { id: "ethanol" },
//     Base { id: "fluorine" },
//     Base { id: "hydrogen" },
//     Base { id: "iodine" },
//     Base { id: "iron" },
//     Base { id: "lithium" },
//     Base { id: "magnesium" },
//     Base { id: "mercury" },
//     Base { id: "nickel" },
//     Base { id: "nitrogen" },
//     Base { id: "oxygen" },
//     Base { id: "phosphorus" },
//     Base { id: "plasma" },
//     Base { id: "platinum" },
//     Base { id: "potassium" },
//     Base { id: "radium" },
//     Base { id: "silicon" },
//     Base { id: "silver" },
//     Base { id: "sodium" },
//     Base { id: "sugar" },
//     Base { id: "sulfur" },
//     Base { id: "water" },
// ];

#[derive(Serialize, Deserialize, Debug)]
pub enum Base {
    Aluminium,
    Barium,
    Bromine,
    Calcium,
    Carbon,
    Chlorine,
    Chromium,
    Copper,
    Ethanol,
    Fluorine,
    Hydrogen,
    Iodine,
    Iron,
    Lithium,
    Magnesium,
    Mercury,
    Nickel,
    Nitrogen,
    Oxygen,
    Phosphorus,
    Plasma,
    Platinum,
    Potassium,
    Radium,
    Silicon,
    Silver,
    Sodium,
    Sugar,
    Sulfur,
    Water,
}

impl Base {
    pub fn get_id(&self) -> String {
        let id = format!("{:?}", self);
        id.to_lowercase()
    }
}

pub static BASES: [Base; 30] = [
    Base::Aluminium,
    Base::Barium,
    Base::Bromine,
    Base::Calcium,
    Base::Carbon,
    Base::Chlorine,
    Base::Chromium,
    Base::Copper,
    Base::Copper,
    Base::Fluorine,
    Base::Hydrogen,
    Base::Iodine,
    Base::Iron,
    Base::Lithium,
    Base::Magnesium,
    Base::Mercury,
    Base::Nickel,
    Base::Nitrogen,
    Base::Oxygen,
    Base::Phosphorus,
    Base::Plasma,
    Base::Platinum,
    Base::Potassium,
    Base::Radium,
    Base::Silicon,
    Base::Silver,
    Base::Sodium,
    Base::Sugar,
    Base::Sulfur,
    Base::Water,
];

#[derive(Serialize, Deserialize, Debug)]
pub struct RawReagent {
    name: String,
    quantity: u32,
}

impl RawReagent {
    pub fn new(name: String, quantity: u32) -> RawReagent {
        RawReagent { name, quantity }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reagent {
    chemical: Chemical,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Data {
    pub compounds: Vec<Compound>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Chemical {
    Base(Base),
    Compound(Compound),
}

impl Chemical {
    pub fn get_id(&self) -> String {
        match self {
            Chemical::Base(base) => base.get_id(),
            Chemical::Compound(compound) => compound.get_id(),
        }
    }   
}

// Finding all of these will be difficult
pub enum Ingredient{
    Fuel,
    FOOF,
}

pub struct ChemTree{
    root: Box<ChemTreeNode>,
}

impl ChemTree {
    pub fn new(root: ChemTreeNode) -> ChemTree{
        ChemTree{
            root: Box::new(root),
        }
    }

    pub fn populate(&self, reagent_map: &HashMap<String, Compound>){
        let id = self.root.get_id();
        println!("Placeholder function for populating tree:\nID: {}\n{:?}", id , reagent_map.get(&id));
    }
}

pub struct ChemTreeNode{
    chemical: Chemical,
    reagents: Box<Vec<Option<ChemTreeNode>>>
}

impl ChemTreeNode {
    pub fn get_id(&self) -> String{
        self.chemical.get_id()
    }
    
}

impl ChemTreeNode {
    pub fn new(chemical: Chemical) -> ChemTreeNode{
        ChemTreeNode { chemical: chemical, reagents: Box::new(Vec::new()) }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compound {
    internal_name: String,
    name: String,
    id: String,
    result: String,
    mix_phrase: String,
    raw_reagents: Vec<RawReagent>,
    required_reagents: Vec<Reagent>,
    result_amount: f32,
    hidden: Option<bool>,
}

impl Compound {
    pub fn new(
        internal_name: String,
        name: String,
        id: String,
        result: String,
        mix_phrase: String,
        raw_reagents: Vec<RawReagent>,
        required_reagents: Vec<Reagent>,
        result_amount: f32,
        hidden: Option<bool>,
    ) -> Compound {
        Compound {
            internal_name,
            name,
            id,
            result,
            mix_phrase,
            raw_reagents,
            required_reagents,
            result_amount,
            hidden,
        }
    }

    //if problems occur change this to get result
    pub fn get_id(&self) -> String{
        self.id.clone()
    }

}