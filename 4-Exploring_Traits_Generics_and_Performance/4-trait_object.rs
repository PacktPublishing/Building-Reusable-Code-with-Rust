pub struct TraitObject {
    pub data: *mut (), // point to the concrete type data
    pub vtable: *mut (), // list of function pointers
}
// https://doc.rust-lang.org/std/raw/struct.TraitObject.html
