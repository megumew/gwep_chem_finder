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
        beaker_type: BeakerType,
        beaker_mode: bool,
        emulate_human: bool,
        easy_beaker: bool,
        perfect_beaker: bool,
        full_reagent_beaker: bool,
    ) -> Self {
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

pub enum BeakerType {
    SmallBeaker,
    LargeBeaker,
    Other,
}

pub fn initialize_settings() -> Settings {
    Settings::default()
}
