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

#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let item_dyn = 
      let item_dn= syn::parse_macro_input!(item as syn::ItemFn);
        let name = &item_dn.sig.ident;
        let body = &item_dn.block;
        let vis = &item_dn.vis;
        let args = &item_dn.sig.inputs;
        let output = &item_dn.sig.output;
        let expanded = quote! {
            #vis fn #name(#args) #output {
                // total time in executing the function
                let start = std::time::Instant::now();
                println!("Entering function: {}", stringify!(#name));
                let result = (|| #body)();
                println!("Exiting function: {}", stringify!(#name));
                let duration = start.elapsed();
                println!("Function {} took {:?}", stringify!(#name), duration);
                result
            }
        };
        TokenStream::from(expanded)

}


#[proc_macro]
pub fn make_map(input : TokenStream) -> TokenStream{
    let input_str = input.to_string();
    let pairs: Vec<&str> = input_str.split(',').collect();
    let mut map_entries = Vec::new();

    for pair in pairs {
        let kv: Vec<&str> = pair.split("=>").map(|s| s.trim()).collect();
        if kv.len() == 2 {
            let key = kv[0];
            let value = kv[1];
            map_entries.push(quote! {
                map.insert(#key.to_string(), #value.to_string());
            });
        }
    }

    let expanded = quote! {
        {
            let mut map = std::collections::HashMap::new();
            #(#map_entries)*
            map
        }
    };

    TokenStream::from(expanded)
}