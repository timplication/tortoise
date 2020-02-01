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
use crate::canvas_engine::Context;
use std::f64::consts::{PI, FRAC_PI_3};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Forward { distance: f64 },
    Left { radians: f64 },
    Right { radians: f64 }
}

pub struct Turtle {
    ops: Vec<Operation>,
    pos: Vec2,
    prev_pos: Vec2,
    home: Vec2,
    rot: f64
}

impl Turtle {
    pub fn new(home: Vec2, dir_rad: f64) -> Turtle {
        Turtle {
            ops: Vec::new(),
            pos: home,
            prev_pos: home,
            home: home,
            rot: dir_rad
        }
    }

    pub fn load(&mut self, mut program: Vec<char>, distance: f64) {
        let mut new_ops: Vec<Operation> = Vec::new();
        while let Some(sym) = program.pop() {
            match sym {
                'A' => new_ops.push(Operation::Forward { distance }),
                'B' => new_ops.push(Operation::Forward { distance }),
                '+' => new_ops.push(Operation::Left { radians: FRAC_PI_3 }),
                '-' => new_ops.push(Operation::Right { radians: FRAC_PI_3 }),
                _   => new_ops.push(Operation::Forward { distance: 0.0 })
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
                Operation::Left { radians } => self.turn(radians),
                Operation::Right { radians } => self.turn(-radians)
            }
        }

        self.ops.clear();
        self.return_home();
    }

    fn return_home(&mut self) {
        self.prev_pos = self.home;
        self.pos = self.home
    }

    fn draw_line(&self, ctx: &mut Context) {
        ctx.begin_path();
        ctx.move_to(self.prev_pos.x, self.prev_pos.y);
        ctx.line_to(self.pos.x, self.pos.y);
        ctx.stroke();
    }

    fn turn(&mut self, radians: f64) {
        // The canvas has a right-handed euclidian coordinate
        // system, we therefor subtract the radians instead
        // of add them, because this converts them from the
        // left-handed system we're used to in math.
        self.rot = (self.rot - radians).rem_euclid(2.0 * PI);
    }

    fn forward(&mut self, distance: f64) {
        self.prev_pos = self.pos;
        self.pos += Vec2::new(distance * self.rot.cos(), distance * self.rot.sin());
    }
}
