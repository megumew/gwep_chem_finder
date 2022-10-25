use std::{
    fs::{self, File},
    path::Path,
};

use std::io::Write;

use serde::Serialize;

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
        beaker_mode: bool,
        percent_toggle: bool,
        beaker_string: String,
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
            profile_name: "DefaultProfile".to_string(),
            percent_toggle: false,
            beaker_type: BeakerType::LargeBeaker,
            beaker_mode: false,
            emulate_human: false,
            easy_beaker: false,
            perfect_beaker: false,
            full_reagent_beaker: false,
        }
    }
}

#[derive(Serialize)]
struct Config {
    settings: SettingsToml,
}

impl Config {
    fn default() -> Self {
        Config {
            settings: SettingsToml::default(),
        }
    }
}

#[derive(Serialize)]
struct SettingsToml {
    profile_name: String,
    beaker_type: String,
    beaker_mode: bool,
    emulate_human: bool,
    easy_beaker: bool,
    perfect_beaker: bool,
    full_reagent_beaker: bool,
}

impl SettingsToml {
    fn default() -> Self {
        SettingsToml {
            profile_name: "DefaultProfile".to_string(),
            beaker_type: "LargeBeaker".to_string(),
            beaker_mode: false,
            emulate_human: false,
            easy_beaker: false,
            perfect_beaker: false,
            full_reagent_beaker: false,
        }
    }
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
        println!("Settings folder exists!");
        if fs::read_dir(path).unwrap().next().is_none() {
            println!("Settings folder is empty.");
            match write_default_config() {
                Ok(_) => println!("Success!"),
                Err(e) => eprintln!("Error occured: {}", e),
            }
        }
    } else {
        println!("Creating settings folder...");
        fs::create_dir_all(path).expect("Failed to create folder!");
        match write_default_config() {
            Ok(_) => println!("Success!"),
            Err(e) => eprintln!("Error occured: {}", e),
        }
    }

    let setting_files = fs::read_dir(path).unwrap();
    let mut setting_choice = Vec::new();

    for element in std::path::Path::new(path).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "toml" {
                setting_choice.push(path);
            }
        }
    }

    for path in setting_choice {
        match path.file_name() {
            Some(name) => println!("Name: {:?}", name),
            None => panic!("Settings file had no name!"),
        }
    }

    Settings::default()
}

fn write_default_config() -> Result<(), std::io::Error> {
    println!("Creating new default config...");
    let default_config_path = format!("{}default_config.toml", SETTINGS_PATH);
    let mut file = File::create(default_config_path).expect("Error while creating file");
    let toml =
        toml::ser::to_string(&Config::default()).expect("Failed to serialize default settings");

    write!(file, "{}", toml)?;
    Ok(())
}
