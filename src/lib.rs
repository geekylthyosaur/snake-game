use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlElement};

#[wasm_bindgen(start)]
pub fn run() {
    let document = window().unwrap().document().unwrap();
    let root = document
        .get_element_by_id("root")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    root
        .style()
        .set_property("background-color", "red")
        .unwrap();
}

