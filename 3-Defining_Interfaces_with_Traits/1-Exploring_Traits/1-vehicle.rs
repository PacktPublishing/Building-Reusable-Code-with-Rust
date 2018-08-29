trait Vehicle {
    fn new(name: &'static str) -> Self; // static method
    fn move(&self) -> (); // instance method
    fn to_string(&self) {
        println!("vehicle {}", self.name) // default implementation
    }
}

struct Airplane { name: &'static str }
struct Bicycle { name: &'static str }
struct Car { name: &'static str }

impl Vehicle for Airplane {
    fn new(name: &'static str) -> Self {
        Airplane { name: name }
    }
    fn move(&self) {
        self.fly();
    }
}

impl Vehicle for Bicycle {
    fn new(name: &'static str) -> Self {
        Bicycle { name: name }
    }
    fn move(&self) {
        self.pedal();
    }
}

impl Vehicle for Car {
    fn new(name: &'static str) -> Self {
        Car { name: name }
    }
    fn move(&self) {
        self.drive();
    }
}
