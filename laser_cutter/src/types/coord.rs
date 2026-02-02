use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Coord(pub(crate) f32, pub(crate) f32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Default for Coord {
    fn default() -> Coord {
        Coord(0.0, 0.0)
    }
}
