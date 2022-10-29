use crate::{
    local::{serialize_to_sql, data_exists},
    parser, sql::{setup_database, database}, fetch::update
};

pub fn initialize(force_update: bool) {
    let update_result = update();

    let updated;
    let path = match update_result {
        (s, b) => {
            updated = b;
            s
        }
    };
    
    let data_string = "data/data.json".to_string();
   
    if updated || !data_exists(&data_string) || force_update {
        setup_database(database());
        let reactions = parser::parse(path);
        println!("There are {} compounds.", reactions.len());
        serialize_to_sql(reactions);
    }
}