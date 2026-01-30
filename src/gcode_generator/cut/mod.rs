use crate::types::{Coord, MachineState};

mod svg_parser;

#[derive(Clone)]
pub enum Segment {
    Line(Coord, Coord),
    Curve
}

#[derive(Clone)]
pub struct Cut {
    pub origin: Coord,
    pub rotation: f32,
    pub cuts: Vec<Segment>
}

impl Cut {
    pub(crate) fn gen_gcode(&self, machine_state: &mut MachineState) -> anyhow::Result<Vec<String>> {
        let mut gcode = vec![];
        for cut in self.cuts.iter() {
            match cut {
                Segment::Line(start, end) => {
                    if *start != machine_state.pos {
                        if machine_state.e {
                            gcode.push("M5".to_string());
                            machine_state.e = false;
                        }
                        gcode.push(format!("G0 X{} Y{}", start.0, start.1));
                       machine_state.pos = *start;
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
                    machine_state.pos = *end;
                }
                Segment::Curve => {}
            }
        }

        gcode.push("M5".to_string());
        machine_state.e = false;
        Ok(gcode)
    }
}

