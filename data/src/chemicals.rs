use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

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
pub struct Ingredient{
    id: String,
}

impl Ingredient {
    pub fn new(id: String) -> Ingredient{
        Ingredient { id }
    }

    pub fn get_id(&self) -> String {
        let id = format!("{}", self.id);
        id.to_lowercase()
    }
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawReagent {
    pub name: String,
    pub quantity: u32,
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
            Chemical::Compound(compound) => compound.get_internal_name(),

        }
    }   
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reaction {
    id: String,
    raw_reagents: Vec<RawReagent>,
    required_reagents: Vec<Reagent>,
    result_amount: f32,
}

impl Reaction {
    pub fn new(
        id: String,
        raw_reagents: Vec<RawReagent>,
        required_reagents: Vec<Reagent>,
        result_amount: f32,
    ) -> Reaction {
        Reaction {
            id,
            raw_reagents,
            required_reagents,
            result_amount,
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Compound {
    internal_name: String,
    name: String,
    result: String,
    reactions: Vec<Reaction>,
    mix_phrase: String,
    required_temperature: Option<f32>,
    instant: bool,
    hidden: bool,
}

impl Compound {
    pub fn new(
        internal_name: String,
        name: String,
        result: String,
        reactions: Vec<Reaction>,
        mix_phrase: String,
        required_temperature: Option<f32>,
        instant: bool,
        hidden: bool,
    ) -> Compound {
        Compound {
            internal_name,
            name,
            result,
            reactions,
            mix_phrase,
            required_temperature,
            instant,
            hidden,
        }
    }

    pub fn get_result(&self) -> String {
        self.result.clone()
    }
    
    pub fn result_amount(&self, u: usize) -> f32 {
        self.reactions[u].result_amount
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_internal_name(&self) -> String {
        self.internal_name.clone()
    }

    pub fn is_instant(&self) -> bool {
        self.instant
    }

    pub fn get_specific_reaction_result_amount(&self, u: usize) -> f32 {
        self.reactions[u].result_amount
    }

    pub fn get_required_temp(&self) -> Option<f32> {
        self.required_temperature
    }

    pub fn get_reagents_of_reaction(&self, u: usize) -> &Vec<RawReagent> {
        &self.reactions[u].raw_reagents
    }

    pub fn get_all_reagents(&self) -> Vec<&Vec<RawReagent>> {
        let mut vec = Vec::new();

        for i in &self.reactions {
            vec.push(&i.raw_reagents)
        };
        vec
    }

    pub fn add_reaction(mut self, new_reaction: Reaction) -> Compound {
        self.reactions.push(new_reaction);
        self
    }

    pub fn recipe_amount(&self) -> usize {
        self.reactions.len()
    }


}