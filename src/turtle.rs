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

use crate::vec_math::Vec2;
use crate::mat_math::Mat2;
use crate::canvas_engine::Context;
use std::f64::consts::{FRAC_PI_4};

#[derive(Clone, Copy)]
enum Operation {
    Forward { distance: f64 },
    Turn { radians: f64 }
}

pub struct Turtle {
    ops: Vec<Operation>,
    pos: Vec2,
    prev_pos: Vec2,
    rot: Mat2
}

impl Turtle {
    pub fn new(pos: Vec2, dir_rad: f64) -> Turtle {
        let mut turtle = Turtle {
            ops: Vec::new(),
            pos: pos,
            prev_pos: pos,
            rot: Mat2::new(0.0, 0.0, 0.0, 0.0)
        };

        turtle.turn(dir_rad);
        turtle
    }

    pub fn load(&mut self, mut program: Vec<char>, distance: f64) {
        let mut new_ops: Vec<Operation> = Vec::new();
        while let Some(sym) = program.pop() {
            match sym {
                'A' => new_ops.push(Operation::Forward { distance }),
                'B' => new_ops.push(Operation::Forward { distance }),
                // Canvas coordinates are flipped for some ungodly reason so we
                // are using unit circle math in australia style here.
                '+' => new_ops.push(Operation::Turn { radians: 7.0 * FRAC_PI_4 }),
                '-' => new_ops.push(Operation::Turn { radians: FRAC_PI_4 }),
                _   => new_ops.push(Operation::Forward { distance: 1.0 })
            }
        }

        self.ops.clear();
        self.ops = new_ops;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for op in self.ops.clone() {
            match op {
                Operation::Forward { distance } => {
                    self.forward(distance);
                    self.draw_line(ctx);
                },
                Operation::Turn { radians } => {
                    self.turn(radians);
                }
            }
        }
    }

    fn draw_line(&self, ctx: &mut Context) {
        ctx.begin_path();
        ctx.move_to(self.prev_pos.x, self.prev_pos.y);
        ctx.line_to(self.pos.x, self.pos.y);
        ctx.stroke();
    }

    fn turn(&mut self, radians: f64) {
        self.rot = Mat2::new(
            radians.cos(), -radians.sin(),
            radians.sin(), radians.cos()
        )
    }

    fn forward(&mut self, distance: f64) {
        let transform = self.rot * distance;
        self.prev_pos = self.pos;
        self.pos = self.rot * self.pos;
    }
}
