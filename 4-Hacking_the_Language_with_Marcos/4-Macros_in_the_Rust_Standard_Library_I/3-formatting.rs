fn main() {
    print!("Print to stdout without newline");
    println!("Print to stdout with newline");
    eprint!("Print to stderr without newline");
    eprintln!("Print to stderr with newline");
    
    println!("My parameter is {}", "foo");
    // My parameter is foo
    println!("{0} {1} {1} {0}", "foo", "bar");
    // foo bar bar foo
    println!("{title}: {author} ({year})", 
             title="Anna Karenina", 
             author="Leo Tolstoy", 
             year="1877");
    // Anna Karenina: Leo Tolstoy (1877)

    let s_format: String = format!("Print to a Stirng with parameter: {}", "foo");
    println!("{}", s_format);
    let s_concat: &str = concat!("i", "can", "haz", "cheezburger");
    println!("{}", s_concat); // icanhazcheezburger
}
