/* Tortoise: A rule-based fractal generator based on
 * turtle graphics & Lindenmayer systems.
 * Copyright (C) 2020 Tim Baccaert <timbaccaert@protonmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

fn get_canvas(elem_id: &str) -> Option<HtmlCanvasElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(elem_id)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()
}

fn get_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas.get_context("2d")
        .ok()
        .flatten()?
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

fn req_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("failed to request animation frame");
}

fn update(mut actions: Box<dyn FnMut()>) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        actions();
        req_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    req_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen(start)]
pub fn start() {
    let canvas = get_canvas("canvas").expect("unable to find canvas element");
    let ctx = get_context(&canvas).expect("unable to get 2d context");
    let mut x = 0.0;

    update(Box::new(move || {
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        ctx.set_fill_style(&"rgb(255,0,0)".into());
        ctx.fill_rect(x, 0.0, 50.0, 50.0);

        x = (((x + 5.0) as u32) % canvas.width()) as f64;
    }));
}
