use std::io;
use std::fs::File;
use std::io::prelude::*;

fn write_to_file_try() -> Result<(), io::Error> {
    // File::create() -> Result<File>
    let mut f = try!(File::create("myfile.txt"));
    f.write_all(b"Hello world");
    Ok(())
}

// equivalent to 
fn write_to_file_match() -> Result<(), io::Error> {
    match File::create("myfile.txt") {
        Ok(mut f) => f.write_all(b"Hello world"), 
        Err(e) => return Err(e) // early return an Err
    }
}

// try! can be shortened to '?'
fn write_to_file_shorthand() -> Result<(), io::Error> {
    File::create("myfile.txt")?.write_all(b"Hello world")
        //                    ^- shorthand for try!
}

fn main() {
    write_to_file_match().unwrap();
    write_to_file_try().unwrap();
    write_to_file_shorthand().unwrap();
}
