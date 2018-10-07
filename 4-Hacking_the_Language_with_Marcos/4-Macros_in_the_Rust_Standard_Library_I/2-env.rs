fn main() {
    let path: &'static str = env!("PATH"); // evaluated at compile time
    println!("the $PATH variable at the time of compiling was: {}", path);
    
    // env! will cause compilation error is not defined.
    let key: Option<&'static str> = option_env!("SECRET_KEY");
    println!("the secret key might be: {:?}", key);
}
