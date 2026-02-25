use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    gcode_generator::operation::{cut::Cut, raster::Raster},
    types::{coord::Coord, machine_settings::MachineState, transform::Transform},
};

pub mod cut;
pub mod raster;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Line(Coord, Coord);

impl Line {
    pub(crate) fn transform(&self, transform: &Transform) -> Line {
        Line(transform.apply(&self.0), transform.apply(&self.1))
    }
}

pub fn deserialize_cuts<'de, D>(data: D) -> Result<Vec<Line>, D::Error>
where
    D: Deserializer<'de>,
{
    let b64 = String::deserialize(data)?;
    let comp_bytes = general_purpose::STANDARD
        .decode(&b64)
        .map_err(serde::de::Error::custom)?;

    let bytes = inflate::inflate_bytes(&comp_bytes).map_err(serde::de::Error::custom)?;
    bitcode::deserialize(&bytes).map_err(serde::de::Error::custom)
}

pub fn serialize_cuts<S>(cuts: &Vec<Line>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let Ok(bytes) = bitcode::serialize(&cuts) else {
        return Err(serde::ser::Error::custom("Failed to serialize"));
    };
    let comp_bytes = deflate::deflate_bytes(&bytes);
    let b64 = general_purpose::STANDARD.encode(&comp_bytes);
    serializer.serialize_str(&b64)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Operation {
    Cut(Cut),
    Raster(Raster),
}

pub trait OperationTrait {
    fn bounds(&self) -> (Coord, Coord);
}

impl Operation {
    pub(crate) fn gen_gcode(
        &self,
        machine_state: &mut MachineState,
    ) -> anyhow::Result<Vec<String>> {
        let (cuts, transform) = match &self {
            Operation::Cut(c) => (&c.cuts, &c.transform),
            Operation::Raster(r) => (&r.cuts, &r.transform),
        };
        let mut gcode = vec![];
        for Line(start, end) in cuts.iter() {
            let start = transform.apply(start);
            let end = transform.apply(end);
            if start != machine_state.pos {
                if machine_state.e {
                    gcode.push("M5".to_string());
                    machine_state.e = false;
                }
                gcode.push(format!("G0 X{} Y{}", start.0, start.1));
                machine_state.pos = start;
            }
            if !machine_state.e {
                gcode.push("M4".to_string());
                machine_state.e = true;
            }
            let mut move_gcode = format!("G1 X{} Y{}", end.0, end.1);
            if machine_state.s != 1000.0 {
                move_gcode.push_str(&format!(" S{}", 1000.0));
                machine_state.s = 1000.0;
            }
            if machine_state.f != 100.0 {
                move_gcode.push_str(&format!(" F{}", 100.0));
                machine_state.f = 100.0;
            }
            gcode.push(move_gcode);
            machine_state.pos = end;
        }

        gcode.push("M5".to_string());
        machine_state.e = false;
        Ok(gcode)
    }

    pub fn bounds(&self) -> (Coord, Coord) {
        match &self {
            Operation::Cut(c) => c.bounds(),
            Operation::Raster(r) => r.bounds(),
        }
    }
}
