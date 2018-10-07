fn main() {
    let current_file = file!();
    let current_line = line!();
    let current_column = column!();
    println!("{}, line {}, column {}", 
             current_file,
             current_line,
             current_column,
            );

    println!("{}", module_path!());
}
