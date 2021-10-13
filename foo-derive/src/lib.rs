extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Foo)]
pub fn derive_foo(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input_name = &input.ident;

    let output = quote! {
        mod generated {
            use super::*;

            struct Dummy {
                inner: #input_name,
            }
        }
    };

    output.into()
}
