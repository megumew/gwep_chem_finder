use reqwest;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

static LINK: &str = "https://raw.githubusercontent.com/goonstation/goonstation/master/code/modules/chemistry/Chemistry-Recipes.dm";
static PATH: &str = "data/Chemistry-Recipes.dm";

//search directory for .dm file, check if it needs updated by comparing with github, and then return the path to the file
pub fn update() -> Result<(String, bool), reqwest::Error> {
    let git_call = reqwest::blocking::get(LINK)?.text();

    let git_file = match git_call {
        Ok(s) => {
            println!("{}", s);
            s
        }
        Err(e) => return Err(e),
    };
    let cur_path = Path::new(PATH);

    if cur_path.exists() {
        let current_file = fs::read_to_string(PATH).expect("cannot read file");
        if !(current_file == git_file) {
            let mut file = File::create(cur_path).expect("Error while creating file");
            write!(file, "{}", git_file).expect("Failed to write to file");
            Ok((String::from(PATH), true))
        } else {
            Ok((String::from(PATH), false))
        }
    } else {
        let mut file = File::create(PATH).expect("Error while creating file");
        write!(file, "{}", git_file).expect("Failed to write to file");
        Ok((String::from(PATH), true))
    }
}
