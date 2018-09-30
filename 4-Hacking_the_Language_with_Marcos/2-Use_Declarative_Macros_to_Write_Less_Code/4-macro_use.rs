// Loads all macros from the "my_module_with_macros" module
#[macro_use]
mod my_module_with_macros;

// Loads only my_macro! from my_crate_with_macros
#[macro_use(my_macro)]
extern crate my_crate_with_macros;
