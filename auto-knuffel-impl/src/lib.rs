use kdl_schema::Schema;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_auto_knuffel(schema: &Schema) -> TokenStream {
    quote! {
        println!("hello world");
    }
}
