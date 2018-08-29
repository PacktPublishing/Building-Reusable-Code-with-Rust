// Make MyNumber and i32 inter-convertible

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

// from i32 to MyNumber
impl From<i32> for MyNumber {
    fn from(number: i32) -> Self {
        MyNumber { value: number }
    }
}

// When we implments From, we get the corresponding 
// "impl Into<MyNumber> for i32" for free

fn main() {
    // Convert an i32 into MyNumber using From<i32>
    let num1 = MyNumber::from(42i32);

    // Convert an i32 into MyNumber using Into<MyNumber>
    let num2: MyNumber = 42i32.into();

    println!("{:?}", num1);
    println!("{:?}", num2);
}
