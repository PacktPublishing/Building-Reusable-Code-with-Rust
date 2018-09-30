macro_rules! vec {
    ( $( $x:expr ),*  ) => {
        { // block
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
