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
mod vec_math;
mod turtle;

use wasm_bindgen::prelude::*;
use canvas_engine::{CanvasEngine, Canvas, Context};
use vec_math::Vec2;
use turtle::Turtle;
use std::f64::consts::*;

struct State {
    turtle: Turtle,
    program: Vec<char>,
    iterations: usize,
    max_iterations: usize
}

fn update(state: &mut State, _: &mut Canvas, _: &mut Context) {
    if state.iterations <= state.max_iterations {
        state.program.reverse();
        state.turtle.load(state.program.clone(), 2.5);

        let mut next_prog: Vec<char> = Vec::new();
        while let Some(sym) = state.program.pop() {
            match sym {
                'A'  => next_prog.append(&mut vec!['B','-','A','-','B']),
                'B'  => next_prog.append(&mut vec!['A','+','B','+','A']),
                s @ _ => next_prog.push(s)
            }
        }

        state.program = next_prog;
        state.iterations += 1;
    }
}

fn render(state: &mut State, cvs: &mut Canvas, ctx: &mut Context) {
    if state.iterations <= state.max_iterations {
        ctx.clear_rect(0.0, 0.0, cvs.width() as f64, cvs.height() as f64);
        ctx.set_line_width(2.0);
        ctx.set_stroke_style(&"rgb(55,55,55)".into());
        state.turtle.draw(ctx);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let home = Vec2::new(145.0, 400.0);
    let turtle = Turtle::new(home, -FRAC_PI_3);
    let program = vec!['A'];
    let state = State { turtle, program, iterations: 0, max_iterations: 8 };
    let engine = CanvasEngine::new("canvas", state, update, render);
    engine.start();
}
