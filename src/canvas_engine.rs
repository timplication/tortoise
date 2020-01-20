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

pub type Canvas = HtmlCanvasElement;
pub type Context = CanvasRenderingContext2d;
pub type Hook<S> = fn(&mut S, &mut Canvas, &mut Context);

pub struct CanvasEngine<S> {
    canvas: Canvas,
    context: Context,
    update: Hook<S>,
    render: Hook<S>,
    state: S
}

impl<S: 'static> CanvasEngine<S> {
    pub fn new(elem_id: &str, state: S, update: Hook<S>, render: Hook<S>) -> CanvasEngine<S> {
        let error = format!("unable to find HTML element with id `{}`", elem_id);
        let canvas = Self::get_canvas(elem_id).expect(&error);
        let context = Self::get_context(&canvas).expect("unable to get 2d context");
        CanvasEngine { canvas, context, update, render, state }
    }

    pub fn start(mut self) {
        let handle = Rc::new(RefCell::new(None));
        let cb = handle.clone();

        *cb.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            (self.update)(&mut self.state, &mut self.canvas, &mut self.context);
            (self.render)(&mut self.state, &mut self.canvas, &mut self.context);
            Self::request_frame(handle.borrow().as_ref().unwrap())
        }) as Box<dyn FnMut()>));

        Self::request_frame(cb.borrow().as_ref().unwrap());
    }

    fn get_canvas(elem_id: &str) -> Option<Canvas> {
        web_sys::window()?
            .document()?
            .get_element_by_id(elem_id)?
            .dyn_into::<Canvas>()
            .ok()
    }

    fn get_context(canvas: &Canvas) -> Option<Context> {
        canvas.get_context("2d")
            .ok()
            .flatten()?
            .dyn_into::<Context>()
            .ok()
    }

    fn request_frame(cb: &Closure<dyn FnMut()>) {
        web_sys::window()
            .and_then(|w| w.request_animation_frame(cb.as_ref().unchecked_ref()).ok())
            .expect("unable to request animation frame");
    }
}
