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

mod canvas_engine;

use wasm_bindgen::prelude::*;
use canvas_engine::{CanvasEngine, Canvas, Context};

struct State(f64);

fn update(state: &mut State, canvas: &mut Canvas, _: &mut Context) {
    state.0 = (((state.0 + 5.0) as u32) % canvas.width()) as f64;
}

fn render(state: &mut State, canvas: &mut Canvas, context: &mut Context) {
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&"rgb(255,0,0)".into());
    context.fill_rect(state.0, 0.0, 50.0, 50.0);
}

#[wasm_bindgen(start)]
pub fn start() {
    let engine = CanvasEngine::new("canvas", State(0.0), update, render);
    engine.start();
}
