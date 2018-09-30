extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation into DeriveInput
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_macro(&ast);

    // Turn the impl_hello_macro output into TokenStream
    gen.parse().unwrap()
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident; // the "identifier", i.e. the "Cat" struct name
    quote! { // turn the code into quote::Tokens
        impl HelloMacro for #name { // template variable #name
            fn hello_macro() {
                println!("Hello, Macro! I'm a {}", stringify!(#name));
            }
        }
    }
}
