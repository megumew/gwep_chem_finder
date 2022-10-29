use serde::{Deserialize, Serialize};

pub static BASES: [&str; 30] = [
    "aluminium",
    "barium",
    "bromine",
    "calcium",
    "carbon",
    "chlorine",
    "chromium",
    "copper",
    "ethanol",
    "fluorine",
    "hydrogen",
    "iodine",
    "iron",
    "lithium",
    "magnesium",
    "mercury",
    "nickel",
    "nitrogen",
    "oxygen",
    "phosphorus",
    "plasma",
    "platinum",
    "potassium",
    "radium",
    "silicon",
    "silver",
    "sodium",
    "sugar",
    "sulfur",
    "water"
];

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reagent {
    name: String,
    quantity: u32,
    ingredient_type: Chemical
}

impl Reagent {
    pub fn new(name: String, quantity: u32, ingredient_type: Chemical) -> Reagent {
        Reagent { name, quantity, ingredient_type }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
    pub fn get_type(&self) -> Chemical {
        self.ingredient_type.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Chemical {
    Base,
    Compound(Reaction),
    Ingredient,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipe {
    id: String,
    raw_reagents: Vec<Reagent>,
    result_amount: f32,
}

impl Recipe {
    pub fn new(id: String, raw_reagents: Vec<Reagent>, result_amount: f32) -> Recipe {
        Recipe {
            id,
            raw_reagents,
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

    pub fn get_id_of_recipe(&self, id: usize) -> String {
        self.recipes[id].id.clone()
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

    pub fn get_mix_phrase(&self) -> String {
        self.mix_phrase.clone()
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn get_reagents_of_recipe(&self, u: usize) -> &Vec<Reagent> {
        &self.recipes[u].raw_reagents
    }

    pub fn get_all_recipes(&self) -> Vec<&Vec<Reagent>> {
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

// #[cfg(test)]
// use crate::initialize_maps::initialize_compound_tree;
// #[test]
// fn test_no_bases_in_reaction_map() {
//     let initialize = initialize_compound_tree(None);
//     let maps = initialize.1;

//     for base in BASES_MAP.clone().into_keys() {
//         match get_reaction(base.to_string()).await? {
//             Ok(_) => panic!("{} should not be in the Reaction Map", base),
//             None => {}
//         }
//     }
// }
// #[test]
// fn test_acetic_acid() {
//     let initialize = initialize_compound_tree(None);
//     let maps = initialize.1;

//     let reaction = get_reaction("acetic_acid").unwrap();

//     let reagents = reaction.get_all_recipes();

//     assert_eq!(reagents.len(), reaction.recipe_amount());
//     assert_eq!(reagents.len(), 1);
//     assert_eq!(reagents[0].len(), 3);
//     assert_eq!(reagents[0][0].name, "acetaldehyde".to_string());
//     assert_eq!(reagents[0][0].quantity, 1);
//     assert_eq!(reagents[0][1].name, "oxygen".to_string());
//     assert_eq!(reagents[0][1].quantity, 1);
//     assert_eq!(reagents[0][2].name, "nitrogen".to_string());
//     assert_eq!(reagents[0][2].quantity, 4);
//     assert_eq!(reaction.result_amount(0), 3.0);
//     assert_eq!(reaction.internal_name, "acetic_acid".to_string());
//     assert_eq!(reaction.name, "Acetic Acid".to_string());
//     assert_eq!(reaction.result, "acetic_acid".to_string());
//     assert_eq!(reaction.recipes[0].id, "acetic_acid".to_string());
//     assert_eq!(reaction.required_temperature, None);
//     assert_eq!(
//         reaction.mix_phrase,
//         "It smells like vinegar and a bad hangover in here.".to_string()
//     );
//     assert_eq!(reaction.instant, false);
//     assert_eq!(reaction.hidden, false);
// }
// #[test]
// fn test_vtonic() {
//     let initialize = initialize_compound_tree(None);
//     let maps = initialize.1;

//     let reaction = maps.reaction_map.get("cocktail_vtonic").unwrap();

//     let reagents = reaction.get_all_recipes();

//     assert_eq!(reagents.len(), reaction.recipe_amount());
//     assert_eq!(reagents.len(), 1);
//     assert_eq!(reagents[0].len(), 2);
//     assert_eq!(reagents[0][0].name, "vodka".to_string());
//     assert_eq!(reagents[0][0].quantity, 1);
//     assert_eq!(reagents[0][1].name, "tonic".to_string());
//     assert_eq!(reagents[0][1].quantity, 1);
//     assert_eq!(reaction.result_amount(0), 2.0);
//     assert_eq!(reaction.internal_name, "cocktail_vtonic".to_string());
//     assert_eq!(reaction.name, "Vodka Tonic".to_string());
//     assert_eq!(reaction.result, "vtonic".to_string());
//     assert_eq!(reaction.recipes[0].id, "vtonic".to_string());
//     assert_eq!(reaction.required_temperature, None);
//     assert_eq!(
//         reaction.mix_phrase,
//         "The tonic water and vodka mix together perfectly.".to_string()
//     );
//     assert_eq!(reaction.instant, false);
//     assert_eq!(reaction.hidden, false);
// }
// #[test]
// fn test_dna_mutagen() {
//     let initialize = initialize_compound_tree(None);
//     let maps = initialize.1;

//     let reaction = maps.reaction_map.get("dna_mutagen").unwrap();

//     let reagents = reaction.get_all_recipes();

//     assert_eq!(reagents.len(), reaction.recipe_amount());
//     assert_eq!(reagents.len(), 2);
//     assert_eq!(reagents[0].len(), 4);
//     assert_eq!(reagents[0][0].name, "mutagen".to_string());
//     assert_eq!(reagents[0][0].quantity, 1);
//     assert_eq!(reagents[0][1].name, "lithium".to_string());
//     assert_eq!(reagents[0][1].quantity, 1);
//     assert_eq!(reagents[0][2].name, "acetone".to_string());
//     assert_eq!(reagents[0][2].quantity, 1);
//     assert_eq!(reagents[0][3].name, "bromine".to_string());
//     assert_eq!(reagents[0][3].quantity, 1);
//     assert_eq!(reagents[1].len(), 2);
//     assert_eq!(reagents[1][0].name, "mutadone".to_string());
//     assert_eq!(reagents[1][0].quantity, 3);
//     assert_eq!(reagents[1][1].name, "lithium".to_string());
//     assert_eq!(reagents[1][1].quantity, 1);
//     assert_eq!(reaction.result_amount(0), 3.0);
//     assert_eq!(reaction.result_amount(1), 4.0);
//     assert_eq!(reaction.internal_name, "dna_mutagen".to_string());
//     assert_eq!(reaction.name, "Stable mutagen".to_string());
//     assert_eq!(reaction.result, "dna_mutagen".to_string());
//     assert_eq!(reaction.recipes[0].id, "dna_mutagen".to_string());
//     assert_eq!(reaction.recipes[1].id, "dna_mutagen2".to_string());
//     assert_eq!(reaction.required_temperature, None);
//     assert_eq!(
//         reaction.mix_phrase,
//         "The substance turns a drab green and begins to bubble.".to_string()
//     );
//     assert_eq!(reaction.instant, false);
//     assert_eq!(reaction.hidden, false);
// }