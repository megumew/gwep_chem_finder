use crate::chemicals::{Chemical, Ingredient, Reaction, BASES_MAP};

use crate::search_engine::Maps;

#[derive(Debug, Clone)]
pub struct ChemTree {
    pub root: Box<ChemTreeNode>,
}

impl ChemTree {
    pub fn new(root: ChemTreeNode) -> ChemTree {
        ChemTree {
            root: Box::new(root),
        }
    }

    pub fn get_compound(&self) -> &Reaction {
        match &self.root.chemical {
            Chemical::Compound(c) => c,
            _ => panic!("A non compound was placed at root of tree!"),
        }
    }

    pub fn populate(&mut self, maps: &Maps) {
        let id = self.root.get_id();
        let chem = Chemical::Compound(maps.reaction_map.get(&id).unwrap().clone());

        let branches = Self::populate_branches(chem, maps);

        self.root.push_root_branches(branches);
    }

    fn populate_branches(chem: Chemical, maps: &Maps) -> Vec<Vec<ChemTreeNode>> {
        let id = chem.get_id();
        let all_recipes = maps.reaction_map.get(&id).unwrap().get_all_recipes();
        let mut top_branch: Vec<Vec<ChemTreeNode>> = Vec::new();

        for raw_reagents in all_recipes {
            let mut branches: Vec<ChemTreeNode> = Vec::new();
            for reagent in raw_reagents {
                let mut reagents: Option<Vec<Vec<ChemTreeNode>>> = None;
                let chemical: Chemical;
                let name = &reagent.name;
                let quantity = reagent.quantity;

                if maps.reaction_map.contains_key(name) {
                    let reaction = maps.reaction_map.get(name).unwrap().clone();
                    chemical = Chemical::Compound(reaction);
                    reagents = Some(Self::populate_branches(chemical.clone(), maps));
                } else if BASES_MAP.contains_key(&name.as_str()) {
                    chemical = Chemical::Base(BASES_MAP.get(&name.as_str()).unwrap().clone());
                } else if maps.result_map.contains_key(name) {
                    let reaction_name = maps.result_map.get(name).unwrap().first().unwrap().clone();
                    let reaction = maps.reaction_map.get(&reaction_name).unwrap().clone();
                    chemical = Chemical::Compound(reaction);
                    reagents = Some(Self::populate_branches(chemical.clone(), maps));
                } else {
                    chemical = Chemical::Ingredient(Ingredient::new(name.clone()));
                }

                let reagent_node = ChemTreeNode::new(quantity as f32, chemical, reagents);

                branches.push(reagent_node);
            }
            top_branch.push(branches);
        }
        top_branch
    }
}

#[derive(Debug, Clone)]
pub struct ChemTreeNode {
    pub chemical: Chemical,
    pub quantity: f32,
    reagents: Box<Option<Vec<Vec<ChemTreeNode>>>>,
}

impl ChemTreeNode {
    pub fn get_id(&self) -> String {
        self.chemical.get_id()
    }

    fn push_root_branches(&mut self, branches: Vec<Vec<ChemTreeNode>>) {
        self.reagents = Box::new(Some(branches));
    }

    pub fn get_reagents(&self) -> &Option<Vec<Vec<ChemTreeNode>>> {
        &self.reagents
    }
}

impl ChemTreeNode {
    pub fn new(
        quantity: f32,
        chemical: Chemical,
        reagents: Option<Vec<Vec<ChemTreeNode>>>,
    ) -> ChemTreeNode {
        ChemTreeNode {
            chemical,
            quantity,
            reagents: Box::new(reagents),
        }
    }
}
