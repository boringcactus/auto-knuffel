use std::mem;

use auto_knuffel_impl::generate_auto_knuffel;
use gloo_events::EventListener;
use kdl_schema::Schema;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn generate_and_format(schema: &Schema) -> String {
    let tokens = generate_auto_knuffel(schema);
    let file = syn::parse2(tokens).unwrap();
    let formatted = prettyplease::unparse(&file);
    formatted
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let miette_handler =
        miette::GraphicalReportHandler::new_themed(miette::GraphicalTheme::unicode_nocolor());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let parse_status = document.create_element("pre")?;
    parse_status.set_inner_html("Enter a schema to parse it and generate auto-knuffel.");
    body.append_child(&parse_status)?;

    let schema = document.create_element("textarea")?;
    let schema = schema
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap_throw();
    body.append_child(&schema)?;

    let generated = document.create_element("pre")?;
    generated.set_inner_html(&generate_and_format(&kdl_schema::SCHEMA_SCHEMA));
    body.append_child(&generated)?;

    let schema_event_target = schema.clone();
    let parse_schema = EventListener::new(&schema_event_target, "input", move |_| {
        let schema_text = schema.value();
        let parse_result = Schema::parse(&schema_text);
        let schema = match parse_result {
            Ok(x) => {
                parse_status.set_inner_html("KDL schema parsed successfully!");
                x
            }
            Err(schema_parse_err) => {
                let mut report = String::new();
                miette_handler
                    .render_report(&mut report, &schema_parse_err)
                    .unwrap_throw();
                parse_status.set_inner_html("");
                parse_status
                    .insert_adjacent_text("afterbegin", &report)
                    .unwrap_throw();
                return;
            }
        };
        let generated_value = generate_and_format(&schema);
        generated.set_inner_html("");
        generated
            .insert_adjacent_text("afterbegin", &generated_value)
            .unwrap_throw();
    });

    mem::forget(parse_schema);

    Ok(())
}
