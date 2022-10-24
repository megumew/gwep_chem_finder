pub struct Settings {
    profile_name: String,
    beaker_type: BeakerType,
    beaker_mode: bool,
    emulate_human: bool,
    easy_beaker: bool,
    perfect_beaker: bool,
    full_reagent_beaker: bool,
}

enum BeakerType {
    SmallBeaker,
    LargeBeaker,
    Other,
}
