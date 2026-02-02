use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Coord(pub(crate) f32, pub(crate) f32);

impl Default for Coord {
    fn default() -> Coord {
        Coord(0.0, 0.0)
    }
}
