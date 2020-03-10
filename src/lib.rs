extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

#[proc_macro_derive(ImplOkHandler)]
pub fn async_route(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of our structure
    let name = input.ident;
    // Get the constraints of our structure
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Verify that we have a structure
    let ts = if let Data::Struct(_s) = input.data {
        // Generate an implementation for our trait
        let expanded = quote! {
            // The generated impl.
            impl #impl_generics OkHandler for #name #ty_generics #where_clause {
                fn ok(&self) -> Result<(), std::io::Error> {
                    Ok(())
                }
            }
        };

        // Convert to stream of tokens to hand back to compiler
        TokenStream::from(expanded)
    } else {
        Error::new(Span::call_site(), "Not a struct").to_compile_error()
    };

    //convert to proc_macro TokenStream
    ts.into()
}
