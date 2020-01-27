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

pub struct Turtle {
    position: Vec2,
    origin: Vec2,
    orientation: Vec2
}

impl Turtle {
    pub fn new() -> Turtle {
        Turtle {
            position: Vec2::new(0.0, 0.0),
            origin: Vec2::new(0.0, 0.0),
            orientation: Vec2::new(0.0, 0.0)
        }
    }

    pub fn turn(&mut self, radians: f64) {
        self.orientation = Vec2::new(radians.cos(), radians.sin());
    }

    pub fn forward(&mut self, distance: f64) {
        self.origin = self.position;
        self.position += self.orientation;
        self.position *= distance;
    }

    pub fn draw(&self, ctx: &mut Context) {
        ctx.begin_path();
        ctx.move_to(self.origin.x, self.origin.y);
        ctx.line_to(self.position.x, self.position.y);
        ctx.stroke();
    }
}
