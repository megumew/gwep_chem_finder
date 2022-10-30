use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use std::io::Write;

use serde::{Deserialize, Serialize};

static SETTINGS_PATH: &str = "settings/";

#[derive(Debug)]
pub struct Settings {
    pub profile_name: String,
    pub percent_toggle: bool,
    pub beaker_mode: bool,
    pub beaker_type: BeakerType,
    pub emulate_human: bool,
    pub easy_beaker: bool,
    pub perfect_beaker: bool,
    pub full_reagent_beaker: bool,
}

impl Settings {
    pub fn new(
        profile_name: String,
        percent_toggle: bool,
        beaker_type: BeakerType,
        beaker_mode: bool,
        emulate_human: bool,
        easy_beaker: bool,
        perfect_beaker: bool,
        full_reagent_beaker: bool,
    ) -> Self {
        Settings {
            profile_name,
            percent_toggle,
            beaker_type,
            beaker_mode,
            emulate_human,
            easy_beaker,
            perfect_beaker,
            full_reagent_beaker,
        }
    }

    pub fn new_from_toml(
        profile_name: String,
        percent_toggle: bool,
        beaker_string: String,
        beaker_mode: bool,
        emulate_human: bool,
        easy_beaker: bool,
        perfect_beaker: bool,
        full_reagent_beaker: bool,
    ) -> Self {
        let beaker_type = match beaker_string.as_str() {
            "SmallBeaker" => BeakerType::SmallBeaker,
            "LargeBeaker" => BeakerType::LargeBeaker,
            "Other" => BeakerType::Other,
            _ => {
                println!("Error reading the beaker type from settings... Please only use SmallBeaker, LargeBeaker, or Other... Defaulting to large beaker");
                BeakerType::LargeBeaker
            }
        };
        Settings {
            profile_name,
            percent_toggle,
            beaker_mode,
            beaker_type,
            emulate_human,
            easy_beaker,
            perfect_beaker,
            full_reagent_beaker,
        }
    }

    pub fn default() -> Self {
        Settings {
            profile_name: "".to_string(),
            percent_toggle: false,
            beaker_type: BeakerType::LargeBeaker,
            beaker_mode: false,
            emulate_human: false,
            easy_beaker: false,
            perfect_beaker: false,
            full_reagent_beaker: false,
        }
    }

    pub fn default_named(name: String) -> Self {
        Settings {
            profile_name: name,
            percent_toggle: false,
            beaker_type: BeakerType::LargeBeaker,
            beaker_mode: false,
            emulate_human: false,
            easy_beaker: false,
            perfect_beaker: false,
            full_reagent_beaker: false,
        }
    }

    pub fn from_toml(settings: SettingsToml) -> Self {
        Settings::new_from_toml(
            settings.profile_name,
            settings.percent_toggle,
            settings.beaker_type,
            settings.beaker_mode,
            settings.emulate_human,
            settings.easy_beaker,
            settings.perfect_beaker,
            settings.full_reagent_beaker,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    settings: SettingsToml,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsToml {
    profile_name: String,
    percent_toggle: bool,
    beaker_type: String,
    beaker_mode: bool,
    emulate_human: bool,
    easy_beaker: bool,
    perfect_beaker: bool,
    full_reagent_beaker: bool,
}

impl SettingsToml {
    fn default_named(profile_name: String) -> Self {
        SettingsToml {
            profile_name,
            percent_toggle: false,
            beaker_type: "LargeBeaker".to_string(),
            beaker_mode: false,
            emulate_human: false,
            easy_beaker: false,
            perfect_beaker: false,
            full_reagent_beaker: false,
        }
    }

    fn from_settings(settings: &Settings) -> Self {
        SettingsToml {
            profile_name: settings.profile_name.clone(),
            percent_toggle: settings.percent_toggle.clone(),
            beaker_type: format!("{:?}", settings.beaker_type),
            beaker_mode: settings.beaker_mode.clone(),
            emulate_human: settings.emulate_human.clone(),
            easy_beaker: settings.easy_beaker.clone(),
            perfect_beaker: settings.perfect_beaker.clone(),
            full_reagent_beaker: settings.full_reagent_beaker.clone(),
        }
    }
}

pub enum YesNo {
    Yes,
    No,
    Default,
    Invalid,
}

#[derive(Debug)]
pub enum BeakerType {
    SmallBeaker,
    LargeBeaker,
    Other,
}

pub fn initialize_settings() -> Settings {
    let path = Path::new(SETTINGS_PATH);

    if path.exists() {
        if fs::read_dir(path).unwrap().next().is_none() {
            println!("Settings folder is empty.");
            return creation_prompt();
        }
    } else {
        println!("Settings folder is missing. Creating settings folder...");
        fs::create_dir_all(path).expect("Failed to create folder!");
        return creation_prompt();
    }

    //let setting_files = fs::read_dir(path).unwrap();
    let mut setting_choice = Vec::new();

    for element in std::path::Path::new(path).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "toml" {
                setting_choice.push(path);
            }
        }
    }

    select_profile(setting_choice)
}

fn create_new_profile() -> Result<Settings, std::io::Error> {
    let mut user_input = String::new();
    println!("Enter the name for your profile (This must be a valid filename):");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
    let profile_name = clean_input(user_input);

    loop {
        println!("Would you like to use default settings? (Y/n)");
        match get_yn() {
            YesNo::Yes => {
                let settings = Settings::default_named(profile_name.clone());
                write_config(&SettingsToml::default_named(profile_name))?;
                return Ok(settings);
            }
            YesNo::No => return Ok(creation_wizard(profile_name)),
            YesNo::Default => {
                let settings = Settings::default_named(profile_name.clone());
                write_config(&SettingsToml::default_named(profile_name))?;
                return Ok(settings);
            }
            YesNo::Invalid => println!("Invalid response!"),
        }
    }
}

fn write_config(settings: &SettingsToml) -> Result<(), std::io::Error> {
    println!("Saving the profile \"{}\"...", settings.profile_name);
    let profile_config_path = format!("{}{}.toml", SETTINGS_PATH, settings.profile_name);
    let mut file = File::create(profile_config_path).expect("Error while creating file");
    let toml = toml::ser::to_string(settings).expect("Failed to serialize default settings");

    write!(file, "{}", toml).expect("Error occured while writing!");

    Ok(())
}

pub fn creation_wizard(profile_name: String) -> Settings {
    println!("Percent toggle (Shows percentage by default): ");
    let precent_toggle = select_bool();

    println!("Beaker Type (Changes the capacity when in beaker mode): ");
    let beaker_type = select_beaker_type();

    println!("Beaker Mode (Shows Beaker mode by default): ");
    let beaker_mode = select_bool();

    println!("Emulate Human (Tries to emulate a human with formula creation): ");
    let emulate_human = select_bool();

    println!("Easy Beaker (Only uses easily input numbers such as 1/5/10): ");
    let easy_beaker = select_bool();

    println!("Perfect Beaker (Creates the beaker as close to max capacity as possible): ");
    let perfect_beaker = select_bool();

    println!("Full Reagent Beaker (Shows the all reagent recipes as full beakers): ");
    let full_reagent_beaker = select_bool();

    let settings = Settings::new(
        profile_name,
        precent_toggle,
        beaker_type,
        beaker_mode,
        emulate_human,
        easy_beaker,
        perfect_beaker,
        full_reagent_beaker,
    );

    write_config(&SettingsToml::from_settings(&settings))
        .expect("Error occured while writing the config!");
    settings
}

fn select_beaker_type() -> BeakerType {
    BeakerType::LargeBeaker
}

fn select_bool() -> bool {
    println!("(1) True\n(2) False");
    let mut user_input = String::new();
    loop {
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => match user_input.trim().parse::<usize>() {
                Ok(i) => match i {
                    1 => return true,
                    2 => return false,
                    _ => println!("Please enter only a valid number!"),
                },
                Err(e) => {
                    println!("Error: {}", e)
                }
            },
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

fn creation_prompt() -> Settings {
    loop {
        println!("Would you like to create a new profile? (y/N)");
        match get_yn() {
            YesNo::Yes => {
                match create_new_profile() {
                    Ok(s) => return s,
                    Err(e) => {
                        println!("Error occured while making profile continuing with default settings: {}", e)
                    }
                }
            }
            YesNo::No => return Settings::default(),
            YesNo::Default => return Settings::default(),
            YesNo::Invalid => {}
        }
    }
}

fn get_yn() -> YesNo {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {}
        Err(e) => panic!("Error: {}", e),
    }
    let clean = clean_input(user_input);

    if clean.is_empty() {
        return YesNo::Default;
    }

    match clean.to_lowercase().chars().next() {
        Some(c) => match c {
            'y' => return YesNo::Yes,
            'n' => return YesNo::No,
            _ => {
                println!("Invalid Response!");
                return YesNo::Invalid;
            }
        },
        None => return YesNo::Default,
    }
}

fn select_profile(setting_choice: Vec<PathBuf>) -> Settings {
    println!("Please select your profile or create a new one:");
    let mut count = 1;
    for path in &setting_choice {
        match path.file_name() {
            Some(name) => {
                println!("({}) {:?}", count, name);
                count += 1;
            }
            None => panic!("Settings file had no name!"),
        }
    }
    println!("({}) Create New Profile", count);

    let mut selection = &PathBuf::new();
    let mut user_input = String::new();
    let mut valid = false;
    let len = setting_choice.len();
    while !valid {
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => match user_input.trim().parse::<usize>() {
                Ok(mut i) => {
                    i -= 1;
                    if i < len {
                        selection = setting_choice.get(i).unwrap();
                        valid = true;
                    } else if i == len {
                        match create_new_profile() {
                            Ok(s) => return s,
                            Err(e) => eprintln!("Error occured: {}", e),
                        }
                    } else {
                        println!(
                            "Please enter only a valid number! (range: 1-{})",
                            setting_choice.len() + 1
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

    retrieve_settings(selection)
}

//expects valid pathbuf
fn retrieve_settings(path: &PathBuf) -> Settings {
    let contents = fs::read_to_string(path).unwrap();
    let settings_toml: SettingsToml = toml::de::from_str(&contents).unwrap();
    Settings::from_toml(settings_toml)
}

fn clean_input(input: String) -> String {
    let words: Vec<_> = input.split_whitespace().collect();
    words.join(" ")
}
