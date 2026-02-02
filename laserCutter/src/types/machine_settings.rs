use serde::{Deserialize, Serialize};

use crate::types::coord::Coord;

#[derive(Serialize, Deserialize)]
pub struct MachineSettings {
    pub min_pos: Coord,
    pub max_pos: Coord,
}

impl Default for MachineSettings {
    fn default() -> Self {
        MachineSettings {
            min_pos: Coord(0.0, 0.0),
            max_pos: Coord(100.0, 100.0),
        }
    }
}

impl MachineSettings {
    pub fn init(max_width: f32, max_height: f32) -> MachineSettings {
        MachineSettings {
            min_pos: Coord(0.0, 0.0),
            max_pos: Coord(max_width, max_height),
        }
    }
}

pub struct MachineState {
    // Position
    pub pos: Coord,
    // Laser
    pub e: bool,
    pub s: f32,
    // Feedrate
    pub f: f32,
}
