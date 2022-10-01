use std::collections::HashMap;

use crate::chemicals::{Compound, Chemical, Ingredient, BASES_MAP};

#[derive(Debug)]
pub struct ChemTree{
    root: Box<ChemTreeNode>,
}

impl ChemTree {
    pub fn new(root: ChemTreeNode) -> ChemTree{
        ChemTree{
            root: Box::new(root),
        }
    }

    fn get_compound(&self) -> &Compound{
        match &self.root.chemical {
            Chemical::Compound(c) => c,
            _ => panic!("A non compound was placed at root of tree!")
        }
    }

    pub fn print_dispenser_format(&self){
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
        println!("----\t{}\t----\n", self.root.get_id().to_uppercase());


        let mut pastable_string = String::new();
        let mut compounds = String::new();
        let mut ingredients = String::new();
        
        for node in self.root.get_reagents() {
            for reagent in node{
                let result = reagent.print_branch(0);
                match result.0{
                    Chemical::Compound(_compound) => {
                        compounds = format!("{}\n{}", compounds, result.1.as_str());
                    }
                    Chemical::Base(_base) => {
                        pastable_string.push_str(result.1.as_str());
                    }
                    Chemical::Ingredient(_ingredient) => {
                        ingredients.push_str(result.1.as_str());
                    }
                }
            }
        }

        match self.get_compound().get_required_temp() {
            Some(temp) => {
                println!("# Required Temperature #");
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                println!("{} K", temp);
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
            }
            None => {}

        }

        if !compounds.is_empty() || !ingredients.is_empty(){
            println!("# Non-base Reagents #");
            println!("+++++++++++++++++++++++++++++++++++++");
    
            if !compounds.is_empty(){
    
                println!("_compounds_");
                println!("{}", compounds);
            }
    
            if !ingredients.is_empty(){
                println!("_ingredients_");
                println!("{}", ingredients);
            }
            println!("+++++++++++++++++++++++++++++++++++++\n");
    
        }
        
        if !pastable_string.is_empty(){
            println!("# Base Reagents #");
            println!("-------------------------------------");
            println!("{}", pastable_string);
            println!("-------------------------------------");
        }

        //self.root.chemical.

        println!("////////////////////////////////////\n");


    }

    pub fn populate(&mut self, compound_map: &HashMap<String, Compound>){

        let id = self.root.get_id();
        let chem = Chemical::Compound(compound_map.get(&id).unwrap().clone());
        
        let branches = Self::populate_branches(chem, compound_map);

        self.root.push_root_branches(branches);
    }

    fn populate_branches(chem: Chemical, compound_map: &HashMap<String, Compound>) -> Vec<ChemTreeNode>{
        let id = chem.get_id();
        let raw_reagents = compound_map.get(&id).unwrap().get_reagents();
        let mut branches: Vec<ChemTreeNode> = Vec::new();
        

        for reagent in raw_reagents{
            let mut reagents: Option<Vec<ChemTreeNode>> = None;
                        let chemical: Chemical;
            let name = &reagent.name;
            let quantity = reagent.quantity;

            if compound_map.contains_key(name){
                chemical = Chemical::Compound(compound_map.get(name).unwrap().clone());
                reagents = Some(Self::populate_branches(chemical.clone(), &compound_map));
            }else if BASES_MAP.contains_key(&name.as_str()){
                chemical = Chemical::Base(BASES_MAP.get(&name.as_str()).unwrap().clone());
            }else{
                chemical = Chemical::Ingredient(Ingredient::new(name.clone()));
            }

            let reagent_node = ChemTreeNode::new(
                quantity as f32,
                chemical,
                reagents
            );

            branches.push(reagent_node);
        }

        branches
    }
}

#[derive(Debug)]
pub struct ChemTreeNode{
    chemical: Chemical,
    quantity: f32,
    reagents: Box<Option<Vec<ChemTreeNode>>>
}

impl ChemTreeNode {
    pub fn get_id(&self) -> String{
        self.chemical.get_id()
    }
    
    fn push_root_branches(&mut self, branches: Vec<ChemTreeNode>){
        self.reagents = Box::new(Some(branches));
    }

    fn get_reagents(&self) -> &Option<Vec<ChemTreeNode>>{
        &self.reagents
    }

    // probably needs to be broken into seperate functions for each reagent type
    fn print_branch(&self, layer: i8) -> (&Chemical, String) {
        let result:(&Chemical, String);

        let mut tab = String::new();
        let mut c = layer;
        while c > 0{
            tab = format!("\t{}", tab);
            c -= 1;
        }

        match &self.chemical{
            Chemical::Compound(compound) => {

                let mut branch_strings = Vec::new();
                for vec in self.get_reagents(){
                    for node in vec{
                        branch_strings.push(node.print_branch(layer + 1));
                    }
                }

                let mut pastable_string = String::new();
                let mut compounds = String::new();
                let mut ingredients = String::new();

                for s in branch_strings{
                    match s.0{
                        Chemical::Compound(_compound) => {
                            compounds = format!("{}\n{}", compounds, s.1.as_str());
                        }
                        Chemical::Base(_base) => {
                            pastable_string.push_str(s.1.as_str());
                        }
                        Chemical::Ingredient(_ingredient) => {
                            ingredients.push_str(s.1.as_str());
                        }
                    }
                }

                let mut branch = String::new();
                if !pastable_string.is_empty(){
                    branch = format!("\n{tab}\t{pastable_string}");
                }
                if !ingredients.is_empty(){
                    branch = format!("{branch}\n{tab}\t{ingredients}");
                }
                if !compounds.is_empty(){
                    branch = format!("{branch}\n{tab}{compounds}");
                }

                let compound_value = format!("{tab}{} {}", self.quantity, compound.get_id().to_uppercase());

                let temp_val = compound.get_required_temp();

                let recipe = match temp_val {
                    Some(temp) => {
                        format!("{} (@{}K)\n{tab}[\n{}\n{tab}]\n", compound_value, temp, branch)
                    }
                    None => {
                    format!("{}\n{tab}[\n{}\n{tab}]\n", compound_value, branch)
                    }
                };


                result = (&self.chemical, recipe);
            }
            Chemical::Base(base) => {
                result = (&self.chemical, format!("{}={};", base.get_id(), self.quantity));
            }
            Chemical::Ingredient(ingredient) => {
                result = (&self.chemical, format!("[{} {}]", self.quantity, ingredient.get_id()));
            }
        }

        result

    }
}

impl ChemTreeNode {
    pub fn new(quantity: f32, chemical: Chemical, reagents: Option<Vec<ChemTreeNode>>) -> ChemTreeNode{
        ChemTreeNode { chemical, quantity,  reagents: Box::new(reagents) }
    }
}