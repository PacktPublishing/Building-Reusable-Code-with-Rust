use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Cat {
    name:  &'static str,
    breed:  &'static str,
    age: u8,
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, 
               "{} the {}-years old {} cat", 
               self.name, 
               self.age, 
               self.breed
              )
    }
}

fn main() {
    let felix = Cat {
        name: "Felix",
        breed: "Scottish Fold",
        age: 4
    };

    let sinba = Cat {
        name: "Sinba",
        breed: "Lion",
        age: 2
    };

    // Display
    println!("{}", felix);
    println!("{}", sinba);

    // Debug
    println!("{:?}", felix);
    println!("{:?}", sinba);

    // Debug, pretty-print
    println!("{:#?}", felix);
    println!("{:#?}", sinba);
}
