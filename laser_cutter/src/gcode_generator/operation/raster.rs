use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    gcode_generator::operation::{Line, OperationTrait, deserialize_cuts, serialize_cuts},
    types::{coord::Coord, transform::Transform},
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Raster {
    pub source: Option<PathBuf>,
    pub transform: Transform,
    #[serde(
        serialize_with = "serialize_cuts",
        deserialize_with = "deserialize_cuts"
    )]
    pub cuts: Vec<Line>,
}

impl Display for Raster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.source {
            Some(ref source) => write!(
                f,
                "Raster: {}",
                source.file_name().unwrap().to_string_lossy()
            ),
            None => write!(f, "Cut: Unknown"),
        }
    }
}

impl OperationTrait for Raster {
    fn bounds(&self) -> (Coord, Coord) {
        todo!()
    }
}
