static BASES: [Base; 30] = [
    Base { id: "aluminium" },
    Base { id: "barium" },
    Base { id: "bromine" },
    Base { id: "calcium" },
    Base { id: "carbon" },
    Base { id: "chlorine" },
    Base { id: "chromium" },
    Base { id: "copper" },
    Base { id: "ethanol" },
    Base { id: "fluorine" },
    Base { id: "hydrogen" },
    Base { id: "iodine" },
    Base { id: "iron" },
    Base { id: "lithium" },
    Base { id: "magnesium" },
    Base { id: "mercury" },
    Base { id: "nickel" },
    Base { id: "nitrogen" },
    Base { id: "oxygen" },
    Base { id: "phosphorus" },
    Base { id: "plasma" },
    Base { id: "platinum" },
    Base { id: "potassium" },
    Base { id: "radium" },
    Base { id: "silicon" },
    Base { id: "silver" },
    Base { id: "sodium" },
    Base { id: "sugar" },
    Base { id: "sulfur" },
    Base { id: "water" },
];

enum Chemical {
    Base(Base),
    Compound(Compound),
}

#[derive(Debug)]
struct Base {
    id: &'static str,
}

struct Compound {
    name: String,
    internal_name: String,
    id: String,
    result: String,
    def_start: usize,
    def_end: usize,
    required_reagents: Vec<Chemical>,
    result_amount: u8,
}

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available bases: {:?}", BASES);
}
