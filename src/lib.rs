use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // test just to see if we can draw stuff
    context.set_fill_style(&"rgb(255,0,0)".into());
    context.fill_rect(200.0, 50.0, 100.0, 100.0);

    context.set_fill_style(&"rgb(0,255,0)".into());
    context.fill_rect(400.0, 50.0, 100.0, 100.0);

    context.set_fill_style(&"rgb(0,0,255)".into());
    context.fill_rect(600.0, 50.0, 100.0, 100.0);

    context.fill();
}
