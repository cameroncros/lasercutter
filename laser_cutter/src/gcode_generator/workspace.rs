use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    gcode_generator::cut::Cut,
    types::{
        coord::Coord,
        gcode::GCode,
        machine_settings::{MachineSettings, MachineState},
    },
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Workspace {
    pub machine_settings: MachineSettings,
    pub items: Vec<Cut>,
}

impl Workspace {
    pub fn init(width: f32, height: f32) -> Workspace {
        Workspace {
            machine_settings: MachineSettings::init(width, height),
            items: vec![],
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Workspace> {
        let file = File::open(path).context("Failed to open file")?;
        let workspace: Workspace =
            serde_json::from_reader(BufReader::new(file)).context("Failed to parse file")?;
        Ok(workspace)
    }

    pub fn save<P: AsRef<Path>>(&self, file_name: P) -> anyhow::Result<()> {
        let file = File::create(file_name).context("Failed to open file")?;
        serde_json::to_writer_pretty(&mut BufWriter::new(file), self)
            .context("Failed to write file")?;
        Ok(())
    }

    pub fn add_cut(&mut self, mut cut: Cut) {
        let (min, _) = cut.bounds();
        cut.transform.offset.0 = -min.0;
        cut.transform.offset.1 = -min.1;
        self.items.push(cut);
    }

    pub fn gen_gcode(&self) -> anyhow::Result<GCode> {
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

    pub fn items(&self) -> &[Cut] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::gcode_generator::{cut::Cut, workspace::Workspace};

    #[test]
    fn test_gen_gcode() {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg("../test_resources/box-all/input.svg".into()).unwrap());

        w.gen_gcode().unwrap().save("out.gcode").unwrap();
    }

    #[test_case("box-all")]
    #[test_case("test_cases")]
    #[test_case("float-issue")]
    fn test_workspace(test_case: &str) {
        let mut initial = Workspace::init(100.0, 100.0);
        initial.add_cut(
            Cut::from_svg(format!("../test_resources/{test_case}/input.svg").into()).unwrap(),
        );

        initial
            .save(format!(
                "../test_resources/{test_case}/actual_workspace.yaml"
            ))
            .unwrap();

        let expected = Workspace::load(format!(
            "../test_resources/{test_case}/actual_workspace.yaml"
        ))
        .unwrap();
        let actual = Workspace::load(format!(
            "../test_resources/{test_case}/expected_workspace.yaml"
        ))
        .unwrap();

        assert_eq!(initial, expected);
        assert_eq!(actual, expected);

        std::fs::remove_file(format!(
            "../test_resources/{test_case}/actual_workspace.yaml"
        ))
        .unwrap();
    }
}
