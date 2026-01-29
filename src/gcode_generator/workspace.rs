use crate::gcode_generator::cut::Cut;
use crate::types::{Coord, GCode, MachineState};

pub struct Segments {}


pub struct Workspace {
    pub width: f32,
    pub height: f32,
    pub items: Vec<Cut>
}

impl Workspace {
    pub fn init(width: f32, height: f32) -> Workspace {
        Workspace {
            width,
            height,
            items: vec![]
        }
    }

    pub(crate) fn add_cut(&mut self, cut: Cut) {
        self.items.push(cut);
    }

    pub(crate) fn gen_gcode(&self) -> anyhow::Result<GCode> {
        let mut items = self.items.clone();
        let mut gcode = vec![
            "G21         ; Set units to mm".to_string(),
            "G90         ; Absolute positioning".to_string(),
        ];
        let mut machine_state = MachineState {
            pos: Coord(0.0, 0.0),
            e: false,
            s: 0.0,
            f: 0.0,
        };

        // FUTURE: Do some kind of ordering to avoid long rapids.
        while let Some(next) = items.pop() {
            let cut_gcode = next.gen_gcode(&mut machine_state)?;
            gcode.extend(cut_gcode);
        }

        Ok(GCode { lines: gcode })
    }
}

#[cfg(test)]
mod tests {
    use crate::gcode_generator::cut::Cut;
    use crate::gcode_generator::workspace::Workspace;

    #[test]
    fn test_gen_gcode() {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg("resources/box-all.svg").unwrap());

        w.gen_gcode().unwrap().save("out.gcode").unwrap();
    }
}
