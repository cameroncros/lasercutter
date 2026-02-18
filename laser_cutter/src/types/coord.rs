use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub},
};

use nalgebra::{Const, Matrix2x1, OMatrix};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Coord(pub(crate) f32, pub(crate) f32);

impl From<Coord> for OMatrix<f32, Const<2>, Const<1>> {
    fn from(coord: Coord) -> OMatrix<f32, Const<2>, Const<1>> {
        Matrix2x1::new(coord.0, coord.1)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(x=")?;
        f.write_fmt(format_args!("{}", self.0))?;
        f.write_str(",y=")?;
        f.write_fmt(format_args!("{}", self.0))?;
        f.write_str(")")
    }
}

// Dot product.
impl Mul<Coord> for Coord {
    type Output = f32;

    fn mul(self, rhs: Coord) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord(rhs.0 * self, rhs.1 * self)
    }
}

impl Add<f32> for Coord {
    type Output = Coord;

    fn add(self, rhs: f32) -> Self::Output {
        Coord(self.0 + rhs, self.1 + rhs)
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Default for Coord {
    fn default() -> Coord {
        Coord(0.0, 0.0)
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn midpoint(first: &Coord, second: &Coord, ratio: f32) -> Coord {
    let delta = *second - *first;
    *first + ratio * delta
}
