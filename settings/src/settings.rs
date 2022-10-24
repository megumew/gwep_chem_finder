use std::{
    fs::{self, File},
    path::Path,
};

use std::io::Write;

use serde::Serialize;

static SETTINGS_PATH: &str = "settings/";

pub struct Settings {
    profile_name: String,
    beaker_type: BeakerType,
    beaker_mode: bool,
    emulate_human: bool,
    easy_beaker: bool,
    perfect_beaker: bool,
    full_reagent_beaker: bool,
}

impl Settings {
    pub fn new(
        profile_name: String,
        beaker_string: String,
        beaker_mode: bool,
        emulate_human: bool,
        easy_beaker: bool,
        perfect_beaker: bool,
        full_reagent_beaker: bool,
    ) -> Self {
        let beaker_type = BeakerType::LargeBeaker;
        Settings {
            profile_name,
            beaker_type,
            beaker_mode,
            emulate_human,
            easy_beaker,
            perfect_beaker,
            full_reagent_beaker,
        }
    }

    pub fn default() -> Self {
        Settings {
            profile_name: "DefaultProfile".to_string(),
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

pub enum BeakerType {
    SmallBeaker,
    LargeBeaker,
    Other,
}

pub fn initialize_settings() -> Settings {
    let path = Path::new(SETTINGS_PATH);

    if path.exists() {
        println!("Settings folder exists!");
        let mut setting_files = fs::read_dir(path).unwrap();
        if setting_files.next().is_none() {
            println!("Settings folder is empty.");
            match write_default_config() {
                Ok(_) => println!("Success!"),
                Err(e) => eprintln!("Error occured: {}", e),
            }
        } else {
            for file in setting_files {
                println!("{:?}", file.unwrap().file_name())
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
