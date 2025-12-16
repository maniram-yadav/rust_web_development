// In my_macro_derive/src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn; // Helper crate for parsing Rust syntax

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a data structure
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");

    // Get the name of the struct/enum
    let name = &ast.ident;

    // Build the output implementation (using the quote crate)
    let code = quote! {
        impl #name {
            fn answer() -> u32 {
                42
            }
        }
    };

    // Convert the generated code back into a TokenStream
    code.into()
}
