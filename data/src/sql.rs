use crate::chemicals::{Chemical, Reaction, Reagent, Recipe, BASES};
use async_recursion::async_recursion;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::ConnectOptions;
use std::collections::HashMap;
use std::str::FromStr;

#[tokio::main]
pub async fn get_all_reactions() -> Result<Vec<Reaction>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut reactions: Vec<Reaction> = Vec::new();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let internal_names = sqlx::query!(
        "
        SELECT internal_name
        FROM reactions;
        "
    )
    .fetch_all(&mut conn)
    .await?;

    for name in internal_names
        .iter()
        .map(|i| i.internal_name.as_ref().unwrap())
    {
        let reaction = get_reaction(name.clone()).await?;

        reactions.push(reaction);
    }
    Ok(reactions)
}

#[async_recursion(?Send)]
async fn get_reaction(internal_name: String) -> Result<Reaction, sqlx::Error> {
    dotenvy::dotenv().ok();

    //std::env::set_var("DATABASE_URL", "sqlite://data/data.db");
    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let recipes = sqlx::query!(
        r#"
        SELECT *
        FROM recipes
        WHERE reaction = ?;
        "#,
        internal_name
    )
    .fetch_all(&mut conn)
    .await?;

    let mut recipe_list: Vec<Recipe> = Vec::new();

    for recipe in recipes {
        let unwraped = recipe.reagents.unwrap();
        let reagents = sqlx::query!(
            r#"
            SELECT *
            FROM reagents
            WHERE recipe = ?;
            "#,
            unwraped
        )
        .fetch_all(&mut conn)
        .await?;
        let mut recipes_reagents: Vec<Reagent> = Vec::new();
        for reagent in reagents {
            let ingredient_type = match reagent.ingredient_type.as_str() {
                "base" => Chemical::Base,
                "compound" => Chemical::Compound(get_reaction(reagent.name.clone()).await?),
                _ => Chemical::Ingredient,
            };
            recipes_reagents.push(Reagent::new(
                reagent.name.clone(),
                reagent.amount as u32,
                ingredient_type,
            ))
        }
        let struc = Recipe::new(recipe.id, recipes_reagents, recipe.result_amount as f32);
        recipe_list.push(struc);
    }
    let reaction_query = sqlx::query!(
        r#"
        SELECT *
        FROM reactions
        WHERE internal_name = ?;
        "#,
        internal_name
    )
    .fetch_one(&mut conn)
    .await?;

    let required_temp: Option<f32>;

    match reaction_query.required_temp {
        Some(temp) => required_temp = Some(temp as f32),
        _ => required_temp = None,
    }

    let reaction = Reaction::new(
        reaction_query.internal_name.unwrap(),
        reaction_query.name,
        reaction_query.result,
        recipe_list,
        reaction_query.mix_phrase,
        required_temp,
        reaction_query.instant,
        reaction_query.hidden,
    );
    Ok(reaction)
}

#[tokio::main]
pub async fn fetch_reaction(internal_name: String) -> Reaction {
    get_reaction(internal_name).await.unwrap()
}

#[tokio::main]
pub async fn add_reaction(reactions: Vec<Reaction>) -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let mut reaction_list: HashMap<String,String> = HashMap::new();

    let mut first_counter: i32 = 0;
    for index in 0..reactions.len() {
        let reaction = reactions[index].clone();
        let internal_name = reaction.get_internal_name();
        let mut name = reaction.get_name().to_lowercase();
        name = name.replace("-", " ");
        let result = reaction.get_result();
        reaction_list.insert(result.clone(), internal_name.clone());
        let mix_phrase = reaction.get_mix_phrase();
        let instant = reaction.is_instant();
        let hidden = reaction.is_hidden();

        sqlx::query!(
            r#"INSERT INTO reactions
            (internal_name, name, result, mix_phrase, instant, hidden)
            VALUES (?,?,?,?,?,?);
            "#,
            internal_name,
            name,
            result,
            mix_phrase,
            instant,
            hidden
        )
        .execute(&mut conn)
        .await?;

        if let Some(temp) = reaction.get_required_temp() {
            sqlx::query!(
                r#"UPDATE reactions
                SET required_temp = ?
                WHERE internal_name = ?
                "#,
                temp,
                internal_name,
            )
            .execute(&mut conn)
            .await?;
        }
        for num in 0..reaction.recipe_amount() {
            let recipe_index = num as i32;
            let id = reaction.get_id_of_recipe(num);
            let result_amount = reaction.get_specific_recipe_result_amount(num);
            sqlx::query!(
                r#"INSERT INTO recipes
                (reaction, recipe_index, id, reagents, result_amount)
                VALUES (?,?,?,?,?);
                "#,
                internal_name,
                recipe_index,
                id,
                first_counter,
                result_amount,
            )
            .execute(&mut conn)
            .await?;

            first_counter += 1;
        }
    }
    let mut second_counter: i32 = 0;
    for index in 0..reactions.len() {
        let reaction = reactions[index].clone();
        for num in 0..reaction.recipe_amount() {
            for reagent in reaction.get_reagents_of_recipe(num) {
                let name = reagent.get_name();
                let amount = reagent.get_quantity();
                sqlx::query!(
                    r#"INSERT INTO reagents
                    (recipe, name, amount)
                    VALUES (?,?,?);
                    "#,
                    second_counter,
                    name,
                    amount,
                )
                .execute(&mut conn)
                .await?;
                if BASES.contains(&name.as_str()) {
                    sqlx::query!(
                        r#"UPDATE reagents
                        SET ingredient_type = 'base'
                        WHERE name LIKE ? AND recipe = ?
                        "#,
                        name,
                        second_counter
                    )
                    .execute(&mut conn)
                    .await?;
                } else if reaction_list.clone().into_values().collect::<Vec<String>>().contains(&name) {
                    sqlx::query!(
                        r#"UPDATE reagents
                        SET ingredient_type = 'compound'
                        WHERE name LIKE ? AND recipe = ?
                        "#,
                        name,
                        second_counter
                    )
                    .execute(&mut conn)
                    .await?;
                } else if reaction_list.contains_key(&name) {
                    let new_name = reaction_list.get(&name).unwrap();
                    sqlx::query!(
                        r#"UPDATE reagents
                        SET ingredient_type = 'compound', 
                        name = ?
                        WHERE name LIKE ? AND recipe = ?
                        "#,
                        new_name,
                        name,
                        second_counter
                    )
                    .execute(&mut conn)
                    .await?;
                } else {
                }
            }
            second_counter += 1;
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn add_reactions(result: Result<(), sqlx::Error>) {
    println!("{:?}", result)
}

#[tokio::main]
pub async fn database() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    //std::env::set_var("DATABASE_URL", "sqlite://data/data.db");
    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    sqlx::query(
        "
        DROP TABLE IF EXISTS reagents;
        DROP TABLE IF EXISTS recipes;
        DROP TABLE IF EXISTS reactions;
        ",
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reactions (
                internal_name TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                result TEXT NOT NULL,
                mix_phrase TEXT NOT NULL,
                required_temp FLOAT,
                instant BOOLEAN NOT NULL,
                hidden BOOLEAN NOT NULL
            );",
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS recipes (
                reaction TEXT,
                reagents INT PRIMARY KEY,
                recipe_index INT NOT NULL,
                id TEXT NOT NULL,
                result_amount FLOAT NOT NULL,
                FOREIGN KEY(reaction) REFERENCES reactions(internal_name)
            );",
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reagents (
                recipe INT,
                name TEXT NOT NULL,
                ingredient_type TEXT NOT NULL DEFAULT 'ingredient',
                amount INT NOT NULL,
                FOREIGN KEY(recipe) REFERENCES recipes(reagents)
            );",
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

#[tokio::main]
pub async fn setup_database(result: Result<(), sqlx::Error>) {
    println!("{:?}", result)
}
