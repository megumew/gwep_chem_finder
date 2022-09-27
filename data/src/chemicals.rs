use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    Base::Ethanol,
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

pub static BASES_MAP: Lazy<HashMap<&str, Base>> = Lazy::new(|| {
    HashMap::from([
    ("aluminium", Base::Aluminium),
    ("barium", Base::Barium),
    ("bromine", Base::Bromine),
    ("calcium", Base::Calcium),
    ("carbon", Base::Carbon),
    ("chlorine", Base::Chlorine),
    ("chromium", Base::Chromium),
    ("copper", Base::Copper),
    ("ethanol", Base::Ethanol),
    ("fluorine", Base::Fluorine),
    ("hydrogen", Base::Hydrogen),
    ("iodine", Base::Iodine),
    ("iron", Base::Iron),
    ("lithium", Base::Lithium),
    ("magnesium", Base::Magnesium),
    ("mercury", Base::Mercury),
    ("nickel", Base::Nickel),
    ("nitrogen", Base::Nitrogen),
    ("oxygen", Base::Oxygen),
    ("phosphorus", Base::Phosphorus),
    ("plasma", Base::Plasma),
    ("platinum", Base::Platinum),
    ("potassium", Base::Potassium),
    ("radium", Base::Radium),
    ("silicon", Base::Silicon),
    ("silver", Base::Silver),
    ("sodium", Base::Sodium),
    ("sugar", Base::Sugar),
    ("sulfur", Base::Sulfur),
    ("water", Base::Water),])
});

// Finding all of these will be difficult
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Ingredient{
    Fuel,
    FOOF,
}

impl Ingredient {
    pub fn get_id(&self) -> String {
        let id = format!("{:?}", self);
        id.to_lowercase()
    }
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawReagent {
    name: String,
    quantity: u32,
}

impl RawReagent {
    pub fn new(name: String, quantity: u32) -> RawReagent {
        RawReagent { name, quantity }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reagent {
    chemical: Chemical,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Data {
    pub compounds: Vec<Compound>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Chemical {
    Base(Base),
    Compound(Compound),
    Ingredient(Ingredient),
}

impl Chemical {
    pub fn get_id(&self) -> String {
        match self {
            Chemical::Base(base) => base.get_id(),
            Chemical::Ingredient(ingredient) => ingredient.get_id(),
            Chemical::Compound(compound) => compound.get_id(),

        }
    }   
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

    pub fn populate(&mut self, compound_map: &HashMap<String, Compound>){
        let id = self.root.get_id();
        let reagents = compound_map.get(&id).unwrap().get_reagents();
        println!("Reagents for ID: {}\n{:?}", id , reagents);

        for reagent in reagents{

            let chemical: Chemical;
            let name = &reagent.name;
            let quantity = reagent.quantity;

            if compound_map.contains_key(name){
                chemical = Chemical::Compound(compound_map.get(name).unwrap().clone());
                // Set the reagents by recursing through the reagents until hitting base or ingredient
            }else if BASES_MAP.contains_key(&name.as_str()){
                chemical = Chemical::Base(BASES_MAP.get(&name.as_str()).unwrap().clone());
            }else{
                // Need to store and lookup ingredients
            }

            // let reagent_node = ChemTreeNode::new(
            //     quantity as f32,
            //     chemical
            // );

            // self.root.reagents.push(Some(reagent_node));
        }
    }
}

pub struct ChemTreeNode{
    chemical: Chemical,
    quantity: f32,
    reagents: Box<Vec<Option<ChemTreeNode>>>
}

impl ChemTreeNode {
    pub fn get_id(&self) -> String{
        self.chemical.get_id()
    }
    
}

impl ChemTreeNode {
    pub fn new(quantity: f32, chemical: Chemical) -> ChemTreeNode{
        ChemTreeNode { chemical, quantity,  reagents: Box::new(Vec::new()) }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

    pub fn get_result_amount(&self) -> f32 {
        self.result_amount.clone()
    }
    
    pub fn get_reagents(&self) -> &Vec<RawReagent> {
        &self.raw_reagents
    }


}