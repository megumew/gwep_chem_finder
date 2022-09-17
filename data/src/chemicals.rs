// static BASES: [Base; 30] = [
//     Base { id: "aluminium" },
//     Base { id: "barium" },
//     Base { id: "bromine" },
//     Base { id: "calcium" },
//     Base { id: "carbon" },
//     Base { id: "chlorine" },
//     Base { id: "chromium" },
//     Base { id: "copper" },
//     Base { id: "ethanol" },
//     Base { id: "fluorine" },
//     Base { id: "hydrogen" },
//     Base { id: "iodine" },
//     Base { id: "iron" },
//     Base { id: "lithium" },
//     Base { id: "magnesium" },
//     Base { id: "mercury" },
//     Base { id: "nickel" },
//     Base { id: "nitrogen" },
//     Base { id: "oxygen" },
//     Base { id: "phosphorus" },
//     Base { id: "plasma" },
//     Base { id: "platinum" },
//     Base { id: "potassium" },
//     Base { id: "radium" },
//     Base { id: "silicon" },
//     Base { id: "silver" },
//     Base { id: "sodium" },
//     Base { id: "sugar" },
//     Base { id: "sulfur" },
//     Base { id: "water" },
// ];

pub static BASES: [Base; 30] = [
    Base::Aluminium,
    Base::Barium,
    Base::Bromine,
    Base::Calcium,
    Base::Carbon,
    Base::Chlorine,
    Base::Chromium,
    Base::Copper,
    Base::Copper,
    Base::Fluorine,
    Base::Hydrogen,
    Base::Iodine,
    Base::Iron,
    Base::Lithium,
    Base::Magnesium,
    Base::Mercury,
    Base::Nickel,
    Base::Nitrogen,
    Base::Oxygen,
    Base::Phosphorus,
    Base::Plasma,
    Base::Platinum,
    Base::Potassium,
    Base::Radium,
    Base::Silicon,
    Base::Silver,
    Base::Sodium,
    Base::Sugar,
    Base::Sulfur,
    Base::Water,
];

#[derive(Debug)]
pub struct Reagent {
    name: Chemical,
    quantity: u32,
}

#[derive(Debug)]
pub enum Chemical {
    Base(Base),
    Compound(Compound),
}

// #[derive(Debug)]
// struct Base {
//     id: &'static str,
// }

#[derive(Debug)]
pub enum Base {
    Aluminium,
    Barium,
    Bromine,
    Calcium,
    Carbon,
    Chlorine,
    Chromium,
    Copper,
    Ethanol,
    Fluorine,
    Hydrogen,
    Iodine,
    Iron,
    Lithium,
    Magnesium,
    Mercury,
    Nickel,
    Nitrogen,
    Oxygen,
    Phosphorus,
    Plasma,
    Platinum,
    Potassium,
    Radium,
    Silicon,
    Silver,
    Sodium,
    Sugar,
    Sulfur,
    Water,
}

// impl fmt::Display for Base {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Base::Aluminium => write!(f, "♥"),
//             Base::Barium => write!(f, "♦"),
//             Base::Bromine => write!(f, "♠"),
//             Base::Calcium => write!(f, "♠"),
//             Base::Carbon => write!(f, "♠"),
//             Base::Chlorine => write!(f, "♠"),
//             Base::Chromium => write!(f, "♠"),
//             Base::Copper => write!(f, "♠"),
//             Base::Ethanol => write!(f, "♠"),
//             Base::Fluorine => write!(f, "♠"),
//             Base::Hydrogen => write!(f, "♠"),
//             Base::Iodine => write!(f, "♠"),
//             Base::Iron => write!(f, "♠"),
//             Base::Lithium => write!(f, "♠"),
//             Base::Magnesium => write!(f, "♠"),
//             Base::Mercury => write!(f, "♠"),
//             Base::Nickel => write!(f, "♠"),
//             Base::Nitrogen => write!(f, "♠"),
//             Base::Oxygen => write!(f, "♠"),
//             Base::Phosphorus => write!(f, "♠"),
//             Base::Plasma => write!(f, "♠"),
//             Base::Platinum => write!(f, "♠"),
//             Base::Potassium => write!(f, "♠"),
//             Base::Radium => write!(f, "♠"),
//             Base::Silicon => write!(f, "♠"),
//             Base::Silver => write!(f, "♠"),
//             Base::Sodium => write!(f, "♠"),
//             Base::Sugar => write!(f, "♠"),
//             Base::Sulfur => write!(f, "♠"),
//             Base::Water => write!(f, "♠"),
//         }
//     }
// }

#[derive(Debug)]
pub struct Compound {
    internal_name: String,
    name: String,
    id: String,
    result: String,
    mix_phrase: String,
    required_reagents: Vec<Reagent>,
    result_amount: f32,
    hidden: Option<bool>,
}

impl Compound {
    pub fn new(
        internal_name: String,
        name: String,
        id: String,
        result: String,
        mix_phrase: String,
        required_reagents: Vec<Reagent>,
        result_amount: f32,
        hidden: Option<bool>,
    ) -> Compound {
        Compound {
            internal_name,
            name,
            id,
            result,
            mix_phrase,
            required_reagents,
            result_amount,
            hidden,
        }
    }
}
