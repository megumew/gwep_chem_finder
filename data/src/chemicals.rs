use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        ("water", Base::Water),
    ])
});

// Finding all of these will be difficult
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ingredient {
    id: String,
}

impl Ingredient {
    pub fn new(id: String) -> Ingredient {
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
    pub compounds: Vec<Reaction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Chemical {
    Base(Base),
    Compound(Reaction),
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
pub struct Recipe {
    id: String,
    raw_reagents: Vec<RawReagent>,
    required_reagents: Vec<Reagent>,
    result_amount: f32,
}

impl Recipe {
    pub fn new(
        id: String,
        raw_reagents: Vec<RawReagent>,
        required_reagents: Vec<Reagent>,
        result_amount: f32,
    ) -> Recipe {
        Recipe {
            id,
            raw_reagents,
            required_reagents,
            result_amount,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reaction {
    internal_name: String,
    name: String,
    result: String,
    recipes: Vec<Recipe>,
    mix_phrase: String,
    required_temperature: Option<f32>,
    instant: bool,
    hidden: bool,
}

impl Reaction {
    pub fn new(
        internal_name: String,
        name: String,
        result: String,
        recipes: Vec<Recipe>,
        mix_phrase: String,
        required_temperature: Option<f32>,
        instant: bool,
        hidden: bool,
    ) -> Reaction {
        Reaction {
            internal_name,
            name,
            result,
            recipes,
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
        self.recipes[u].result_amount
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

    pub fn get_specific_recipe_result_amount(&self, u: usize) -> f32 {
        self.recipes[u].result_amount
    }

    pub fn get_required_temp(&self) -> Option<f32> {
        self.required_temperature
    }

    pub fn get_reagents_of_recipe(&self, u: usize) -> &Vec<RawReagent> {
        &self.recipes[u].raw_reagents
    }

    pub fn get_all_recipes(&self) -> Vec<&Vec<RawReagent>> {
        let mut vec = Vec::new();

        for i in &self.recipes {
            vec.push(&i.raw_reagents)
        }
        vec
    }

    pub fn add_recipe(mut self, new_recipe: Recipe) -> Reaction {
        self.recipes.push(new_recipe);
        self
    }

    pub fn recipe_amount(&self) -> usize {
        self.recipes.len()
    }
}

#[cfg(test)]
    use crate::initialize_maps::initialize_compound_tree;
    #[test]
    fn test_no_bases_in_reaction_map() {
        let initialize = initialize_compound_tree("data.json".to_string(), None);
        let maps = initialize.1;

        for base in BASES_MAP.clone().into_keys() {
            match maps.reaction_map.get(&base.to_string()) {
                Some(_) => panic!("{}", base),
                None => {}
            }
        }
    }
    #[test]
    fn test_acetic_acid() {
        let initialize = initialize_compound_tree("data.json".to_string(), None);
        let maps = initialize.1;

        let reaction = maps.reaction_map.get("acetic_acid").unwrap();

        let reagents = reaction.get_all_recipes();

        assert_eq!(reagents.len(), reaction.recipe_amount());
        assert_eq!(reagents.len(), 1);
        assert_eq!(reagents[0].len(), 3);
        assert_eq!(reagents[0][0].name, "acetaldehyde".to_string());
        assert_eq!(reagents[0][0].quantity, 1);
        assert_eq!(reagents[0][1].name, "oxygen".to_string());
        assert_eq!(reagents[0][1].quantity, 1);
        assert_eq!(reagents[0][2].name, "nitrogen".to_string());
        assert_eq!(reagents[0][2].quantity, 4);
        assert_eq!(reaction.result_amount(0), 3.0);
        assert_eq!(reaction.internal_name, "acetic_acid".to_string());
        assert_eq!(reaction.name, "Acetic Acid".to_string());
        assert_eq!(reaction.result, "acetic_acid".to_string());
        assert_eq!(reaction.recipes[0].id, "acetic_acid".to_string());
        assert_eq!(reaction.required_temperature, None);
        assert_eq!(reaction.mix_phrase, "It smells like vinegar and a bad hangover in here.".to_string());
        assert_eq!(reaction.instant, false);
        assert_eq!(reaction.hidden, false);
    }
    #[test]
    fn test_vtonic() {
        let initialize = initialize_compound_tree("data.json".to_string(), None);
        let maps = initialize.1;

        let reaction = maps.reaction_map.get("cocktail_vtonic").unwrap();

        let reagents = reaction.get_all_recipes();

        assert_eq!(reagents.len(), reaction.recipe_amount());
        assert_eq!(reagents.len(), 1);
        assert_eq!(reagents[0].len(), 2);
        assert_eq!(reagents[0][0].name, "vodka".to_string());
        assert_eq!(reagents[0][0].quantity, 1);
        assert_eq!(reagents[0][1].name, "tonic".to_string());
        assert_eq!(reagents[0][1].quantity, 1);
        assert_eq!(reaction.result_amount(0), 2.0);
        assert_eq!(reaction.internal_name, "cocktail_vtonic".to_string());
        assert_eq!(reaction.name, "Vodka Tonic".to_string());
        assert_eq!(reaction.result, "vtonic".to_string());
        assert_eq!(reaction.recipes[0].id, "vtonic".to_string());
        assert_eq!(reaction.required_temperature, None);
        assert_eq!(reaction.mix_phrase, "The tonic water and vodka mix together perfectly.".to_string());
        assert_eq!(reaction.instant, false);
        assert_eq!(reaction.hidden, false);
    }
    #[test]
    fn test_dna_mutagen() {
        let initialize = initialize_compound_tree("data.json".to_string(), None);
        let maps = initialize.1;

        let reaction = maps.reaction_map.get("dna_mutagen").unwrap();

        let reagents = reaction.get_all_recipes();

        assert_eq!(reagents.len(), reaction.recipe_amount());
        assert_eq!(reagents.len(), 2);
        assert_eq!(reagents[0].len(), 4);
        assert_eq!(reagents[0][0].name, "mutagen".to_string());
        assert_eq!(reagents[0][0].quantity, 1);
        assert_eq!(reagents[0][1].name, "lithium".to_string());
        assert_eq!(reagents[0][1].quantity, 1);
        assert_eq!(reagents[0][2].name, "acetone".to_string());
        assert_eq!(reagents[0][2].quantity, 1);
        assert_eq!(reagents[0][3].name, "bromine".to_string());
        assert_eq!(reagents[0][3].quantity, 1);
        assert_eq!(reagents[1].len(), 2);
        assert_eq!(reagents[1][0].name, "mutadone".to_string());
        assert_eq!(reagents[1][0].quantity, 3);
        assert_eq!(reagents[1][1].name, "lithium".to_string());
        assert_eq!(reagents[1][1].quantity, 1);
        assert_eq!(reaction.result_amount(0), 3.0);
        assert_eq!(reaction.result_amount(1), 4.0);
        assert_eq!(reaction.internal_name, "dna_mutagen".to_string());
        assert_eq!(reaction.name, "Stable mutagen".to_string());
        assert_eq!(reaction.result, "dna_mutagen".to_string());
        assert_eq!(reaction.recipes[0].id, "dna_mutagen".to_string());
        assert_eq!(reaction.recipes[1].id, "dna_mutagen2".to_string());
        assert_eq!(reaction.required_temperature, None);
        assert_eq!(reaction.mix_phrase, "The substance turns a drab green and begins to bubble.".to_string());
        assert_eq!(reaction.instant, false);
        assert_eq!(reaction.hidden, false);
    }
