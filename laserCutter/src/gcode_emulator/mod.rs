mod renderer;
mod svg_renderer;

use anyhow::bail;

use crate::{
    gcode_emulator::{
        Positioning::RELATIVE,
        renderer::{RenderSettings, Renderer},
        svg_renderer::SvgRenderer,
    },
    types::{coord::Coord, gcode::GCode, machine_settings::MachineState},
};

enum Positioning {
    RELATIVE,
    ABSOLUTE,
}

pub struct GCodeEmulator {
    gcode: GCode,
    state: MachineState,
    current_line: usize,
    positioning: Positioning,
    renderer: SvgRenderer,
}

impl GCodeEmulator {
    pub fn from_file(file: &str) -> anyhow::Result<GCodeEmulator> {
        let gcode = GCode::load(file)?;
        GCodeEmulator::from_gcode(gcode)
    }

    pub fn from_gcode(gcode: GCode) -> anyhow::Result<GCodeEmulator> {
        Ok(GCodeEmulator {
            gcode,
            state: MachineState {
                pos: Coord::default(),
                e: false,
                s: 0.0,
                f: 0.0,
            },
            current_line: 0,
            positioning: Positioning::ABSOLUTE,
            renderer: SvgRenderer::new(),
        })
    }

    pub fn step(&mut self) -> anyhow::Result<bool> {
        let next_real = loop {
            let next = match self.gcode.lines.get(self.current_line) {
                Some(line) => line.trim(),
                None => return Ok(false),
            };
            self.current_line += 1;
            if next.starts_with(";") || next.is_empty() {
                continue;
            }
            break next;
        };

        let parts = next_real.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "G0" => {
                /* Move to X/Y rapid? */
                let (x, y, _, _) = parse_args(&parts[1..])?;
                self.renderer.draw_line(
                    self.state.pos,
                    Coord(x, y),
                    RenderSettings {
                        color: "red".to_string(),
                        thickness: 0.1,
                        opacity: 0.5,
                    },
                )?;
                self.state.pos = Coord(x, y);
            }
            "G1" => {
                /* Move to X/Y, with feed and power */
                let (x, y, _, p) = parse_args(&parts[1..])?;
                self.renderer.draw_line(
                    self.state.pos,
                    Coord(x, y),
                    RenderSettings {
                        color: "green".to_string(),
                        thickness: 0.1,
                        opacity: p / 1000f32,
                    },
                )?;
                self.state.pos = Coord(x, y);
            }
            "G21" => { /* Set units to mm */ }
            "G90" => self.positioning = RELATIVE,
            "M4" => { /* Laser On */ }
            "M5" => { /* Laser Off */ }
            _ => bail!("Unknown code: {}", next_real),
        }

        Ok(true)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while self.step()? {}
        Ok(())
    }

    pub fn save(&mut self, file: &str) -> anyhow::Result<()> {
        self.renderer.save(file)
    }
}

fn parse_args(args: &[&str]) -> anyhow::Result<(f32, f32, f32, f32)> {
    let mut x = 0f32;
    let mut y = 0f32;
    let mut s = 0f32;
    let mut f = 0f32;

    for arg in args {
        match arg.chars().next().unwrap() {
            'X' | 'x' => x = arg[1..].parse()?,
            'Y' | 'y' => y = arg[1..].parse()?,
            'S' | 's' => s = arg[1..].parse()?,
            'F' | 'f' => f = arg[1..].parse()?,

            _ => bail!("Unknown argument: {}", arg),
        }
    }

    Ok((x, y, s, f))
}

#[cfg(test)]
mod tests {
    use crate::gcode_emulator::GCodeEmulator;

    #[test]
    fn laser_gcode_test() {
        let mut gce = GCodeEmulator::from_file("resources/test.gcode").unwrap();
        gce.run().unwrap();
        gce.save("out.svg").unwrap();
    }
}
