#[derive(Default, Debug)]
struct MyData {
    int_field: i32,
    float_field: f32,
}

fn main() {
    // Use default values for all the fields
    let data1: MyData = Default::default();
    println!("{:?}", data1);

    // Specify part of the fields and use the defaults for the rest
    let data2 = MyData {
        int_field: 42,
        ..Default::default() // Use the default for the rest 
    };
    println!("{:?}", data2);
}
