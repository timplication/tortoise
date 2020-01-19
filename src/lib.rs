use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};

fn req_frame(f: &Closure<dyn FnMut()>) {
    window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn canvas(html_id: &str) -> HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(html_id).unwrap();

    canvas.dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

fn setup_loop(mut update: Box<dyn FnMut()>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        update();
        req_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    req_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen(start)]
pub fn start() {
    let canvas = canvas("canvas");
    let ctx = context(&canvas);
    let mut x = 0.0;

    setup_loop(Box::new(move || {
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        ctx.set_fill_style(&"rgb(255,0,0)".into());
        ctx.fill_rect(x, 0.0, 50.0, 50.0);

        x = (((x + 5.0) as u32) % canvas.width()) as f64;
    }));
}
