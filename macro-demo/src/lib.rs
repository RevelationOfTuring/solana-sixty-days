extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

// Declaring a procedural attribute-like macro using the `proc_macro_attribute` directive
// This makes the macro usable as an attribute
#[proc_macro_attribute]
// The function `my_custom_attribute` takes two arguments:
// _metadata: The arguments provided to the macro (if any)
// _input: The TokenStream the macro is applied to
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into an AST node representing a struct
    let input = parse_macro_input!(_input as ItemStruct);
    let struct_name = &input.ident; // Get the name of the struct

    // Constructing the output TokenStream using the quote! macro
    // The quote! macro allows for writing Rust code as if it were a string,
    // but with the ability to interpolate values
    TokenStream::from(quote! {
        // Derive Debug trait for #struct_name to enable formatted output with `println()`
        #[derive(Debug)]
        // Defining a new struct #struct_name with two fields: `i1` and `i2`
        struct #struct_name {
            i1: i32,
            i2: i32,
        }

        // Implementing the Default trait for #struct_name
        // This provides a default() method to create a new instance of #struct_name
        impl Default for #struct_name {
            // The default method returns a new instance of #struct_name
            // with `i1`` set to 1024 and `i2`` set to 2048
            fn default() -> Self {
                #struct_name { i1: 1024, i2: 2048}
            }
        }

        impl #struct_name {
            // Defining a method double_i1 for #struct_name
            // This method returns double the value of i1
            fn double_i1(&self) -> i32 {
                self.i1 * 2
            }
        }
    })
}

#[proc_macro_attribute]
pub fn delete_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as ItemStruct);
    let struct_name = &input.ident; // Get the name of the struct

    TokenStream::from(quote! {
        // This returns an empty struct with the same name
        // Delete all fields in the struct
        #[derive(Debug)]
        struct #struct_name {
        }
    })
}
