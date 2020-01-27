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
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Mat2 {
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64
}

impl Mat2 {
    pub fn new(a11: f64, a12: f64, a21: f64, a22: f64) -> Mat2 {
        Mat2 { a11, a12, a21, a22 }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.a11 * v.x + self.a12 * v.y,
            self.a21 * v.x + self.a22 * v.y
        )
    }
}

impl Mul<f64> for Mat2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Mat2 {
        Mat2::new(
            self.a11 * scalar, self.a12 * scalar,
            self.a21 * scalar, self.a22 * scalar
        )
    }
}
