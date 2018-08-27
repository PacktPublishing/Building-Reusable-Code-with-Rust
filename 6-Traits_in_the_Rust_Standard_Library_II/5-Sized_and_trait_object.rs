trait NoisyAnimal: Sized { 
    //             ^---- Sized trait can't be made into a trait object
    fn make_noise(&self) -> &'static str;
}


struct Cat {}
impl NoisyAnimal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

struct Dog {}
impl NoisyAnimal for Dog {
    fn make_noise(&self) -> &'static str{
        "woof"
    }
}

fn make_noises(animals: Vec<Box<NoisyAnimal>>) {
    //                          ^---------- Trait Object
    for animal in animals.iter() {
        println!("{}", animal.make_noise());
    }
}

fn main() {
    let animals: Vec<Box<NoisyAnimal>> = vec![
        Box::new(Dog{}),
        Box::new(Cat{}),
    ];
    make_noises(animals);
}
