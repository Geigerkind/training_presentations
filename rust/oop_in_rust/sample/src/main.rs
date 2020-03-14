trait Animal {
    fn get_name(&self) -> String;
}

trait AnimalSound : Animal {
    fn play_sound(&self);
}

struct AnimalPictureBook {
    pub name: String,
    pub pages: Vec<Box<dyn AnimalSound>>
}


struct Cat {}
struct Dog {}
struct RubberDuck {}

impl AnimalSound for Cat {
    fn play_sound(&self) {
        println!("Miau!");
    }
}
impl Animal for Cat {
    fn get_name(&self) -> String {
        "Cat".to_owned()
    }
}

impl AnimalSound for Dog {
    fn play_sound(&self) {
        println!("Wuff!");
    }
}
impl Animal for Dog {
    fn get_name(&self) -> String {
        "Dog".to_owned()
    }
}

impl AnimalSound for RubberDuck {
    fn play_sound(&self) {
        println!("*Silence*");
    }
}
impl Animal for RubberDuck {
    fn get_name(&self) -> String {
        "RubberDuck".to_owned()
    }
}

fn main() {
    let book = AnimalPictureBook {
        name: "Fantastic animals and their sounds".to_owned(),
        pages: vec![
            Box::new(Cat {}),
            Box::new(Dog {}),
            Box::new(RubberDuck {})
        ]
    };

    println!("The name of the book is {}", book.name);
    for page in book.pages.iter() {
        println!("Continuing to next page...");
        println!("{} says:", page.get_name());
        page.play_sound();
        println!("");
    }
}
