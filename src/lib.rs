extern crate proc_macro;
use proc_macro::TokenStream;

use auto_knuffel_impl::generate_auto_knuffel;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn auto_knuffel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    // TODO read the input file as a schema

    let tokens = generate_auto_knuffel(&kdl_schema::SCHEMA_SCHEMA);

    tokens.into()
}
