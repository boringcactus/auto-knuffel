use auto_knuffel_impl::generate_auto_knuffel;
use kdl_schema::Schema;
use wasm_bindgen::prelude::*;

fn generate_and_format(schema: &Schema) -> String {
    let tokens = generate_auto_knuffel(schema);
    let file = syn::parse2(tokens).unwrap();
    let formatted = prettyplease::unparse(&file);
    formatted
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("pre")?;
    val.set_inner_html(&generate_and_format(&kdl_schema::SCHEMA_SCHEMA));

    body.append_child(&val)?;

    Ok(())
}
