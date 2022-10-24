use reqwest;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

static LINK: &str = "https://raw.githubusercontent.com/goonstation/goonstation/master/code/modules/chemistry/Chemistry-Recipes.dm";
static PATH: &str = "data/Chemistry-Recipes.dm";

//search directory for .dm file, check if it needs updated by comparing with github, and then return the path to the file
pub fn update() -> (String, bool) {
    let path_string = String::from(PATH);
    let cur_path = Path::new(PATH);

    let git_file: String = match download_script() {
        Ok(s) => s,
        // Add more useful error checking
        Err(e) => {
            eprintln!("Error updating... :{}", e);
            if cur_path.exists() {
                return (path_string, false);
            } else {
                panic!("Unable to fetch needed script. Program will not function without being to update script at least once.")
            }
        }
    };

    if cur_path.exists() {
        let current_file = fs::read_to_string(PATH).expect("cannot read file");
        if !(current_file == git_file) {
            let mut file = File::create(cur_path).expect("Error while creating file");
            write!(file, "{}", git_file).expect("Failed to write to file");
            (path_string, true)
        } else {
            (path_string, false)
        }
    } else {
        fs::create_dir_all("data/").expect("Failed to create folder!");
        let mut file = File::create(PATH).expect("Error while creating file");
        write!(file, "{}", git_file).expect("Failed to write to file");
        (path_string, true)
    }
}

fn download_script() -> Result<String, reqwest::Error> {
    let git_call = reqwest::blocking::get(LINK)?.text();

    let git_file = match git_call {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    return Ok(git_file);
}
