use data::{
    chem_tree::{ChemTree, ChemTreeNode},
    chemicals::Chemical,
};

pub fn print_dispenser_format(tree: ChemTree, show_percent: bool) {
    for node in tree.root.get_reagents() {
        let mut count = 1;
        for recipe in node {
            println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
            if node.len() > 1 {
                println!("----\t{}\t----", tree.root.get_id().to_uppercase());
                println!("\tRECIPE #{}\n", count);
                count += 1;
            } else {
                println!("----    {}    ----\n", tree.root.get_id().to_uppercase());
            }

            let mut bases = String::new();
            let mut compounds = String::new();
            let mut ingredients = String::new();
            for reagent in recipe {
                // I heard u liked one-liners... This gets the percent each reagent is of the top chem
                let percent =  if  show_percent {
                    (100.0 * (100.0 / tree.root.get_reagents().as_ref().unwrap()[0]
                    .iter()
                    .fold(0.0, |a, b| a + b.quantity)).round())/100.0
                } else {
                    reagent.quantity as f32
                };
                let result = print_branch(reagent.clone(), 0, percent, show_percent);
                match result.0 {
                    Chemical::Compound(_) => {
                        compounds = format!("{}\n{}", compounds, result.1.as_str());
                    }
                    Chemical::Base(_) => {
                        bases.push_str(result.1.as_str());
                    }
                    Chemical::Ingredient(_) => {
                        ingredients.push_str(result.1.as_str());
                    }
                }
            }

            match tree.get_compound().get_required_temp() {
                Some(temp) => {
                    println!("# Required Temperature #");
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                    println!("{} K", temp);
                    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
                }
                None => {}
            }

            if !compounds.is_empty() || !ingredients.is_empty() {
                println!("# Non-base Reagents #");
                println!("+++++++++++++++++++++++++++++++++++++");

                if !compounds.is_empty() {
                    println!("_____________________________________");
                    println!("Compounds");
                    println!("-------------------------------------");
                    println!("{}", compounds);
                }

                if !ingredients.is_empty() {
                    println!("___________________________________");
                    println!("Ingredients");
                    println!("-----------------------------------");
                    println!("{}", ingredients);
                }
                println!("+++++++++++++++++++++++++++++++++++++\n");
            }

            if !bases.is_empty() {
                println!("_____________________________________");
                println!("Base Reagents");
                println!("-------------------------------------");
                println!("{}", bases);
                println!("-------------------------------------");
            }

            println!("////////////////////////////////////////////////////////////////////////\n");
        }
    }
}

// probably needs to be broken into seperate functions for each reagent type
fn print_branch(branch: ChemTreeNode, layer: i8, percent: f32, show_percent: bool) -> (Chemical, String) {
    let result: (&Chemical, String);

    let mut tab = String::new();
    let mut c = layer;
    while c > 0 {
        tab = format!("\t{}", tab);
        c -= 1;
    }

    match &branch.chemical {
        Chemical::Compound(compound) => {
            let mut branch_strings = Vec::new();
            for top_branch in branch.get_reagents() {
                let recipe = &top_branch[0]; // Moved Hardcoded use of 1st Recipe here
                for node in recipe {
                    if !show_percent {
                        branch_strings.push(print_branch(
                            node.clone(),
                            layer + 1,
                            node.quantity,
                            show_percent
                        ));
                    } else {
                        branch_strings.push(print_branch(
                            node.clone(),
                            layer + 1,
                            ((100.0 * percent/(node.quantity * (recipe.iter().fold(0.0, |a, b| a + b.quantity)))).round())/100.0 ,
                            show_percent
                        ));
                    }
                }
            }

            let mut bases = String::new();
            let mut compounds = String::new();
            let mut ingredients = String::new();

            for s in branch_strings {
                match s.0 {
                    Chemical::Compound(_) => {
                        compounds = format!("{}\n{}", compounds, s.1.as_str());
                    }
                    Chemical::Base(_) => {
                        bases.push_str(s.1.as_str());
                    }
                    Chemical::Ingredient(_) => {
                        ingredients.push_str(s.1.as_str());
                    }
                }
            }

            let mut new_branch = String::new();
            if !bases.is_empty() {
                new_branch = format!("\n{tab}\t{}", bases);
            }
            if !ingredients.is_empty() {
                new_branch = format!("{new_branch}\n{tab}\t{}", ingredients);
            }
            if !compounds.is_empty() {
                new_branch = format!("{new_branch}\n{tab}{}", compounds);
            }

            let compound_value = 
                if show_percent {
                    format!(
                        "{tab}[{}% {}]",
                        percent,
                        compound.get_internal_name().to_ascii_uppercase()
                    )
                } else {
                    format!(
                        "{tab}[{} {}]",
                        percent,
                        compound.get_internal_name().to_ascii_uppercase()
                    )
                };

            let temp_val = compound.get_required_temp();

            let recipe = match temp_val {
                Some(temp) => {
                    format!(
                        "{} (@{}K)\n{tab}{{\n{}\n{tab}}}\n",
                        compound_value, temp, new_branch
                    )
                }
                None => {
                    format!("{}\n{tab}{{\n{}\n{tab}}}\n", compound_value, new_branch)
                }
            };

            result = (&branch.chemical, recipe);
        }
        Chemical::Base(base) => {
            if show_percent {
                result = (
                    &branch.chemical,
                    format!(
                        "({}% {}) ",
                        percent,
                        base.get_id().to_ascii_uppercase()
                    ),
                );
            } else {
                result = (
                    &branch.chemical,
                    format!(
                        "({} {}) ",
                        percent,
                        base.get_id().to_ascii_uppercase()
                    ),
                );
            }
        }
        Chemical::Ingredient(ingredient) => {
            if show_percent {
                result = (
                    &branch.chemical,
                    format!(
                        "<{}%\"{}\"> ",
                        percent,
                        ingredient.get_id()
                    ),
                );
            } else {
                result = (
                    &branch.chemical,
                    format!(
                        "<{}\"{}\"> ",
                        percent,
                        ingredient.get_id()
                    ),
                );
            }
        }
    }
    return (result.0.clone(), result.1);
}
