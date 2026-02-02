use std::fmt::{Display, Formatter};

use anyhow::bail;
use clap::Parser;
use dialoguer::Select;
use laser_cutter::{
    gcode_emulator::GCodeEmulator,
    gcode_generator::{cut::Cut, workspace::Workspace},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Args {
    LoadWorkspace {
        #[clap(short, long)]
        input_file: String,
        #[clap(short, long)]
        output_file: String,
    },
    Console {},
}

#[derive(Debug)]
enum Tasks {
    LOAD,
    NEW,
    SAVE,
    QUIT,
    ADD,
    DELETE,
    MOVE,
    ROTATE,
    SCALE,
    GENERATE,
}

impl Display for Tasks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args {
        Args::LoadWorkspace {
            input_file,
            output_file,
        } => {
            let w = Workspace::load(&input_file)?;
            let gcode = w.gen_gcode()?;

            let mut gce = GCodeEmulator::from_gcode(gcode)?;
            gce.run()?;
            gce.save(&output_file)
        }
        Args::Console {} => {
            let mut workspace: Option<Workspace> = None;

            loop {
                let modes = if workspace.is_none() {
                    vec![Tasks::LOAD, Tasks::NEW, Tasks::QUIT]
                } else {
                    vec![
                        Tasks::SAVE,
                        Tasks::QUIT,
                        Tasks::ADD,
                        Tasks::DELETE,
                        Tasks::MOVE,
                        Tasks::ROTATE,
                        Tasks::SCALE,
                        Tasks::GENERATE,
                    ]
                };
                let mode = Select::new().items(&modes).default(0).interact()?;
                let selected_mode = &modes[mode];
                match selected_mode {
                    Tasks::NEW => {
                        workspace = Some(Workspace::init(100.0, 100.0));
                    }
                    Tasks::LOAD => {
                        let file_name: String = dialoguer::Input::new()
                            .with_prompt("File name")
                            .interact()?;
                        workspace = Some(Workspace::load(&file_name)?);
                    }
                    Tasks::SAVE => {
                        let file_name: String = dialoguer::Input::new()
                            .with_prompt("File name")
                            .interact()?;
                        if let Some(workspace) = workspace.as_mut() {
                            workspace.save(&file_name)?;
                        } else {
                            bail!("No workspace loaded");
                        }
                    }
                    Tasks::QUIT => {
                        return Ok(());
                    }
                    Tasks::ADD => {
                        let file_name: String = dialoguer::Input::new()
                            .with_prompt("File name")
                            .interact()?;
                        if let Some(workspace) = workspace.as_mut() {
                            workspace.add_cut(Cut::from_svg(&file_name)?);
                        } else {
                            bail!("No workspace loaded");
                        }
                    }
                    Tasks::DELETE => {}
                    Tasks::MOVE => {}
                    Tasks::ROTATE => {}
                    Tasks::SCALE => {}
                    Tasks::GENERATE => {
                        let file_name: String = dialoguer::Input::new()
                            .with_prompt("File name")
                            .interact()?;
                        if let Some(workspace) = workspace.as_mut() {
                            workspace.gen_gcode()?.save(&file_name)?;
                        } else {
                            bail!("No workspace loaded");
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use laser_cutter::{
        gcode_emulator::GCodeEmulator,
        gcode_generator::{cut::Cut, workspace::Workspace},
    };
    use test_case::test_case;

    #[test_case("box-all")]
    #[test_case("test_cases")]
    fn end_to_end_test(input_file: &str) {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg(&format!("resources/{input_file}.svg")).unwrap());
        w.gen_gcode()
            .unwrap()
            .save(&format!("gcode_{input_file}.gcode"))
            .unwrap();
        w.save(&format!("workspace_{input_file}.yaml")).unwrap();

        let mut gce = GCodeEmulator::from_file(&format!("gcode_{input_file}.gcode")).unwrap();
        gce.run().unwrap();
        gce.save(&format!("output_{input_file}.svg")).unwrap();
    }
}
