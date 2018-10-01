fn main() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    assert_eq!(vec, [1, 2, 3, 4]);
    
    let vec = vec![0; 5];
    //              ^ ';' not ','
    assert_eq!(vec, [0, 0, 0, 0, 0]);
}
