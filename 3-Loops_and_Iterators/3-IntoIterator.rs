fn main() {
    let mylist = vec!(1, 2, 3, 4);
    
    for element in mylist {
        // .into_iter() can be omitted
        println!("{}", element);
    }

    /*
    let mut iter = IntoIterator::into_iter(mylist);
    loop {
        match iter.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }
    */
}
