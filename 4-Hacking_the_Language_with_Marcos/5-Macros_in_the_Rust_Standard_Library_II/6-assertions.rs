fn main() {
    assert!(1 == 1);

    let x = true;
    assert!(x, "x isn't true!");

    assert_eq!(1, 1);

    let y = 10;
    let z = 20;
    assert_eq!(y, z)


    // Only run in un-optimized builds
    debug_assert!(1 == 1);
    debug_assert_eq!(1, 1);
}
