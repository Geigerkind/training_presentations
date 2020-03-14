#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
enum DegreeType {
    Science,
    Arts,
    Business
}

#[derive(Debug)]
enum Degree {
    Bachelor { degree_type: DegreeType },
    Master { degree_type: DegreeType },
    PHD(String)
}

#[derive(Debug)]
enum ProgrammerState {
    Eating,
    Sleeping,
    Programming
}

#[derive(Debug)]
enum Beverage {
    Coffee(bool),
    Beer { promille: Option<f64> }
}

#[derive(Debug)]
struct Programmer {
    pub name: Option<String>,
    pub age: Option<u8>,
    pub degree: Option<Degree>,
    pub state: ProgrammerState,
    pub beverage: Beverage
}

impl Default for Programmer {
    fn default() -> Programmer {
        Programmer {
            name: None,
            age: None,
            degree: None,
            state: ProgrammerState::Programming,
            beverage: Beverage::Coffee(true)
        }
    }
}

impl Programmer {
    pub fn print_degree(&self) {
        match self.degree.as_ref() {
            None => println!("{} has no Degree!", self.name.as_ref().unwrap()),
            Some(degree) => println!("{} has a {}", self.name.as_ref().unwrap(), degree)
        }
    }
}


impl fmt::Display for DegreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DegreeType::Science => write!(f, "Science"),
            DegreeType::Arts => write!(f, "Arts"),
            DegreeType::Business => write!(f, "Business")
        }
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Degree::Bachelor { degree_type } => write!(f, "Bachelor of {}!", degree_type),
            Degree::Master { degree_type } => write!(f, "Master of {}!", degree_type),
            Degree::PHD(title) => write!(f, "PHD in '{}'!", title)
        }
    }
}

fn main() {
    let mut programmer_1 = Programmer::default();
    programmer_1.name = Some("Bob".to_owned());
    programmer_1.degree = Some(Degree::Master { degree_type: DegreeType::Science });

    let mut programmer_2 = Programmer::default();
    programmer_2.name = Some("Alice".to_owned());
    programmer_2.degree = Some(Degree::PHD("Software Security".to_owned()));

    println!("Alice: {:?}", programmer_2);
    println!("Bob: {:?}", programmer_1);

    println!("");

    programmer_1.print_degree();
    programmer_2.print_degree();
}
