use crate::{
    fetch::update,
    local::{data_exists, serialize_to_sql},
    parser,
    sql::{database, setup_database},
};

pub fn initialize(force_update: bool) {
    std::env::set_var("GWEP_DATABASE_URL", "sqlite://data/data.db");
    let update_result = update();

    let updated;
    let path = match update_result {
        (s, b) => {
            updated = b;
            s
        }
    };

    let data_string = "data/data.db".to_string();

    if updated || !data_exists(&data_string) || force_update {
        setup_database(database());
        let reactions = parser::parse(path);
        println!("There are {} compounds.", reactions.len());
        serialize_to_sql(reactions);
    }
}
