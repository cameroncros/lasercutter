use serde::{Deserialize, Serialize};

use crate::types::coord::Coord;

#[derive(Clone, Serialize, Deserialize)]
pub struct Transform {
    pub(crate) rotate: (f32, f32, f32, f32),
    pub(crate) offset: Coord,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotate: (1.0, 0.0, 0.0, 1.0),
            offset: Coord(0.0, 0.0),
        }
    }
}

impl Transform {
    pub(crate) fn apply(&self, coord: &Coord) -> Coord {
        let Coord(x, y) = coord;
        let Coord(ox, oy) = self.offset;
        Coord(
            (self.rotate.0 * x + self.rotate.2 * x) + ox,
            (self.rotate.1 * y + self.rotate.3 * y) + oy,
        )
    }
}
