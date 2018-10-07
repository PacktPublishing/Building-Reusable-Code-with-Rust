fn main() {
    let file_output = include!("external_rust_code.txt");
    println!("{}", file_output);

    let file_string = include_str!("external_string.txt");
    println!("{}", file_string);

    // let byte_array: &'static [u8; N] = include_bytes!("external_string.txt");
}
