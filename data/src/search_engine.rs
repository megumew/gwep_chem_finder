use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions,
};
use std::{io, str::FromStr};

#[tokio::main]
pub async fn reagent_uses(reagent: String) -> Result<Vec<String>, sqlx::Error> {
    let mut strings: Vec<String> = Vec::new();

    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let search = sqlx::query!(
        r#"
        SELECT recipe
        FROM reagents
        WHERE name LIKE ?;
        "#,
        reagent
    )
    .fetch_all(&mut conn)
    .await?;

    for recipe in search {
        let search_recipe = sqlx::query!(
            r#"
            SELECT reaction
            FROM recipes
            WHERE reagents LIKE ?;
            "#,
            recipe.recipe
        )
        .fetch_all(&mut conn)
        .await?;

        for reaction in search_recipe {
            strings.push(reaction.reaction.unwrap())
        }
    }

    Ok(strings)
}

#[tokio::main]
pub async fn reaction_search(input: &String) -> Result<Vec<String>, sqlx::Error> {
    let mut strings: Vec<String> = Vec::new();

    let mut clean = input.to_string();
    if input.len() > 10 {
        clean.truncate(10)
    }
    strings = search_reaction_starts_with(&clean, strings).await?;
    strings = search_reaction_multi_starts_with(&clean, strings).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    }

    strings = search_reaction_contains(&clean, strings).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    }

    strings = search_typos(&clean, strings, true).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    } else if strings.len() > 0 {
        return Ok(strings);
    }

    Err(sqlx::Error::RowNotFound)
}

async fn search_reaction_starts_with(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!("{}%", input);

    let internal_name_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE internal_name LIKE ?
        ORDER BY internal_name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let result_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE result LIKE ?
        ORDER BY result ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let name_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE name LIKE ?
        ORDER BY name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    for output in internal_name_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in result_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in name_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    Ok(strings)
}

async fn search_reaction_multi_starts_with(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!(r"%\_{}%", input);

    let underscore_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE internal_name LIKE ? ESCAPE '\'
        OR result LIKE ? ESCAPE '\'
        ORDER BY internal_name ASC;
        "#,
        formatted,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let formatted_space = format!("% {}%", input);

    let space_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE name LIKE ?
        OR result LIKE ?
        ORDER BY name,result ASC;
        "#,
        formatted_space,
        formatted_space
    )
    .fetch_all(&mut conn)
    .await?;

    let formatted_hyphen = format!("%-{}%", input);

    let hyphen_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE name LIKE ?
        ORDER BY internal_name ASC;
        "#,
        formatted_hyphen
    )
    .fetch_all(&mut conn)
    .await?;

    for output in underscore_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in space_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in hyphen_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    Ok(strings)
}

async fn search_reaction_contains(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!("%{}%", input);

    let internal_name_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE internal_name LIKE ?
        ORDER BY internal_name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let result_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE result LIKE ?
        ORDER BY result ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let name_search = sqlx::query!(
        r#"
        SELECT internal_name
        FROM reactions
        WHERE name LIKE ?
        ORDER BY name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    for output in internal_name_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in result_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }
    for output in name_search {
        let unwrapped = output.internal_name.unwrap();
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    Ok(strings)
}

/*
- Replaces characters in input with '_' representing any possible single character (or lack thereof), then searches through all other searches
- Checks in reverse order of string, so if searching for "blood", searching "bloop", "bloed", etc. will give better results than "flood".
- Likewise, searching "flood", a single letter typo, will give better results than "bloea", "blaad", "breod", two letter typos.
- Catches incorrect string lengths as well, but their priority is lowered.
- Searching "blooood" hits "blood" on loop 2-2, due to looking up "bloo__d" hitting "blood"
- Typo search is O(n²) at minimum until it hits at least 5 results so input length is limited to 10 characters
*/
async fn search_typos(
    input: &String,
    mut strings: Vec<String>,
    reaction: bool,
) -> Result<Vec<String>, sqlx::Error> {
    // Prevents underflow
    // Was thinking about doing input / x but doing this and truncating the input when it's received leads to a wider range of results
    let mut reserved_length = input.len() as i32 - 2;
    if reserved_length < 0 {
        reserved_length = 0
    }

    for length in 0..reserved_length as usize {
        for index in (length..input.len()).rev() {
            let mut new_input = input.clone();
            new_input.replace_range(index - length..index + 1, "_");

            if reaction {
                strings = search_reaction_starts_with(&new_input, strings).await?;
                strings = search_reaction_multi_starts_with(&new_input, strings).await?;
                strings = search_reaction_contains(&new_input, strings).await?;
            } else {
                strings = search_reagent_starts_with(&new_input, strings).await?;
                strings = search_reagent_multi_starts_with(&new_input, strings).await?;
                strings = search_reagent_contains(&new_input, strings).await?;
            }

            if strings.len() >= 5 {
                return Ok(strings[0..5].to_vec());
            }
        }
    }

    Ok(strings)
}

#[tokio::main]
pub async fn reagent_search(input: &String) -> Result<Vec<String>, sqlx::Error> {
    let mut strings: Vec<String> = Vec::new();

    let mut clean = input.to_string();
    if input.len() > 10 {
        clean.truncate(10)
    }

    strings = search_reagent_starts_with(&clean, strings).await?;
    strings = search_reagent_multi_starts_with(&clean, strings).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    }

    strings = search_reagent_contains(&clean, strings).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    }

    strings = search_typos(&clean, strings, false).await?;

    if strings.len() > 5 {
        return Ok(strings[0..5].to_vec());
    } else if strings.len() > 0 {
        return Ok(strings);
    }

    Err(sqlx::Error::RowNotFound)
}

async fn search_reagent_starts_with(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!("{}%", input);

    let name_search = sqlx::query!(
        r#"
        SELECT name
        FROM reagents
        WHERE name LIKE ?
        ORDER BY name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    for output in name_search {
        let unwrapped = output.name;
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    Ok(strings)
}

async fn search_reagent_multi_starts_with(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!(r"%\_{}%", input);

    let underscore_search = sqlx::query!(
        r#"
        SELECT name
        FROM reagents
        WHERE name LIKE ? ESCAPE '\'
        ORDER BY name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    let formatted2 = format!(r"% {}%", input);

    let space_search = sqlx::query!(
        r#"
        SELECT name
        FROM reagents
        WHERE name LIKE ? ESCAPE '\'
        ORDER BY name ASC;
        "#,
        formatted2
    )
    .fetch_all(&mut conn)
    .await?;

    let formatted3 = format!(r"%-{}%", input);

    let hyphen_search = sqlx::query!(
        r#"
        SELECT name
        FROM reagents
        WHERE name LIKE ? ESCAPE '\'
        ORDER BY name ASC;
        "#,
        formatted3
    )
    .fetch_all(&mut conn)
    .await?;

    for output in underscore_search {
        let unwrapped = output.name;
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    for output in space_search {
        let unwrapped = output.name;
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    for output in hyphen_search {
        let unwrapped = output.name;
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    Ok(strings)
}

async fn search_reagent_contains(
    input: &String,
    mut strings: Vec<String>,
) -> Result<Vec<String>, sqlx::Error> {
    dotenvy::dotenv().ok();

    let env = &std::env::var("GWEP_DATABASE_URL").ok().unwrap();

    let mut conn = SqliteConnectOptions::from_str(env)?
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await?;

    let formatted = format!("%{}%", input);

    let contains_search = sqlx::query!(
        r#"
        SELECT name
        FROM reagents
        WHERE name LIKE ?
        ORDER BY name ASC;
        "#,
        formatted
    )
    .fetch_all(&mut conn)
    .await?;

    for output in contains_search {
        let unwrapped = output.name;
        if !strings.contains(&unwrapped) {
            strings.push(unwrapped)
        }
    }

    Ok(strings)
}

pub fn collision_select(result: &Vec<String>) -> String {
    println!(
        "Found {} likely options. Please select one to continue.",
        result.len()
    );
    for (i, r) in result.iter().enumerate() {
        println!("{}. {}", i + 1, r);
    }

    let mut selection = String::new();
    let mut valid = false;
    while !valid {
        let mut i_num = String::new();
        match io::stdin().read_line(&mut i_num) {
            Ok(_) => match i_num.trim().parse::<usize>() {
                Ok(mut i) => {
                    i -= 1;
                    if i < result.len() {
                        selection = result.get(i).unwrap().to_string();
                        println!("Selecting {} ({})", i + 1, selection);
                        valid = true;
                    } else {
                        println!(
                            "Please enter only a valid number! (range {}-{})",
                            1,
                            result.len()
                        );
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            },
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    selection
}
