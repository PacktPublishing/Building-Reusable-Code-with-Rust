trait X {}
trait Y {}
trait Z: X + Y {}

struct A;

//impl X for A {}
//impl Y for A {}
impl Z for A {}

fn main() {}
