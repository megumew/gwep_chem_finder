use data::chemicals::{Chemical, Reaction};

#[tokio::main]
pub async fn print_dispenser_format(reaction: Reaction, show_percent: bool) {
    let mut count = 1;
    for recipe in reaction.get_all_recipes() {
        println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
        println!("----\t{}\t----", reaction.get_internal_name().to_uppercase());
        if reaction.recipe_amount() > 1 {
            println!("\tRECIPE #{}\n", count);
            count += 1;
        }

        let mut bases = String::new();
        let mut compounds = String::new();
        let mut ingredients = String::new();
        for reagent in recipe {
            // This gets the percent each reagent is of the top chem if enabled
            let percent = if show_percent {
                reagent.get_quantity() as f32 * ((10000.0 / recipe.iter()
                    .fold(0.0, |a, b| a + b.get_quantity() as f32))
                    .round()
                )/ 100.0
            } else {
                reagent.get_quantity() as f32
            };
            let result = print_branch(reagent.clone(), 0, percent, show_percent);
            match result.0 {
                Chemical::Compound(_) => {
                    compounds = format!("{}\n{}", compounds, result.1.as_str());
                }
                Chemical::Base => {
                    bases.push_str(result.1.as_str());
                }
                Chemical::Ingredient => {
                    ingredients.push_str(result.1.as_str());
                }
            }
        }

        match reaction.get_required_temp() {
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

// probably needs to be broken into seperate functions for each reagent type
fn print_branch(
    reagent: data::chemicals::Reagent,
    layer: i8,
    percent: f32,
    show_percent: bool,
) ->  (Chemical, String) {
    let result: (Chemical, String);

    let mut tab = String::new();
    let mut c = layer;
    while c > 0 {
        tab = format!("\t{}", tab);
        c -= 1;
    }

    match reagent.get_type() {
        Chemical::Compound(reaction) => {
            let mut branch_strings = Vec::new();
            let all_reagents = reaction.get_reagents_of_recipe(0);
            for lower_reagent in all_reagents {
                if !show_percent {
                    branch_strings.push(print_branch(
                        lower_reagent.clone(),
                        layer + 1,
                        lower_reagent.get_quantity() as f32,
                        show_percent,
                    ));
                } else {
                    branch_strings.push(print_branch(
                        lower_reagent.clone(),
                        layer + 1,
                        ((100.0 * percent
                            / (lower_reagent.get_quantity() as f32
                                * (all_reagents.iter().fold(0.0, |a, b| a + b.get_quantity() as f32))))
                        .round())
                            / 100.0,
                        show_percent,
                    ));
                }
            }

            let mut bases = String::new();
            let mut compounds = String::new();
            let mut ingredients = String::new();

            for s in branch_strings {
                let clone = s;
                match clone.0 {
                    Chemical::Compound(_) => {
                        compounds = format!("{}\n{}", compounds, clone.1.as_str());
                    }
                    Chemical::Base => {
                        bases.push_str(clone.1.as_str());
                    }
                    Chemical::Ingredient => {
                        ingredients.push_str(clone.1.as_str());
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

            let compound_value = if show_percent {
                format!(
                    "{tab}[{}% {}]",
                    percent,
                    reaction.get_internal_name().to_ascii_uppercase()
                )
            } else {
                format!(
                    "{tab}[{} {}]",
                    percent,
                    reaction.get_internal_name().to_ascii_uppercase()
                )
            };

            let temp_val = reaction.get_required_temp();

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

            result = (Chemical::Compound(reaction), recipe);
        }
        Chemical::Base => {
            if show_percent {
                result = (
                    Chemical::Base,
                    format!("({}% {}) ", percent, reagent.get_name().to_ascii_uppercase()),
                );
            } else {
                result = (
                    Chemical::Base,
                    format!("({} {}) ", percent, reagent.get_name().to_ascii_uppercase()),
                );
            }
        }
        Chemical::Ingredient => {
            if show_percent {
                result = (
                    Chemical::Ingredient,
                    format!("<{}%\"{}\"> ", percent, reagent.get_name()),
                );
            } else {
                result = (
                    Chemical::Ingredient,
                    format!("<{}\"{}\"> ", percent, reagent.get_name()),
                );
            }
        }
    }
    return result;
}
