use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{coord::Coord, machine_settings::MachineState, transform::Transform};

mod svg_parser;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Segment {
    Line(Coord, Coord),
    Curve,
}

fn deserialize_cuts<'de, D>(data: D) -> Result<Vec<Segment>, D::Error>
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

fn serialize_cuts<S>(cuts: &Vec<Segment>, serializer: S) -> Result<S::Ok, S::Error>
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Cut {
    pub source: Option<String>,
    pub transform: Transform,
    #[serde(
        serialize_with = "serialize_cuts",
        deserialize_with = "deserialize_cuts"
    )]
    pub cuts: Vec<Segment>,
}

impl Cut {
    pub(crate) fn gen_gcode(
        &self,
        machine_state: &mut MachineState,
    ) -> anyhow::Result<Vec<String>> {
        let mut gcode = vec![];
        for cut in self.cuts.iter() {
            match cut {
                Segment::Line(start, end) => {
                    let start = self.transform.apply(start);
                    let end = self.transform.apply(end);
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
                Segment::Curve => {}
            }
        }

        gcode.push("M5".to_string());
        machine_state.e = false;
        Ok(gcode)
    }

    pub fn bounds(&self) -> (Coord, Coord) {
        let mut min = Coord(f32::MAX, f32::MAX);
        let mut max = Coord(f32::MIN, f32::MIN);
        for segment in self.cuts.iter() {
            match segment {
                Segment::Line(start, end) => {
                    let start = self.transform.apply(start);
                    let end = self.transform.apply(end);
                    min.0 = min.0.min(start.0);
                    min.0 = min.0.min(end.0);
                    min.1 = min.1.min(start.1);
                    min.1 = min.1.min(end.1);
                    max.0 = max.0.max(start.0);
                    max.0 = max.0.max(end.0);
                    max.1 = max.1.max(start.1);
                    max.1 = max.1.max(end.1);
                }
                Segment::Curve => {}
            }
        }
        (min, max)
    }
}
