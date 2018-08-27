#[allow(dead_code)]

#[derive(Debug)]
enum Pet {
    Cat,
    Dog,
    Bird,
}

impl Default for Pet {
    fn default() -> Pet {
        Pet::Cat
    }
}

fn main() {
    let default_pet: Pet = Default::default();

    println!("{:?}", default_pet);
}
