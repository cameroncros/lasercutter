#[cfg(not(any(feature = "svg_renderer", feature = "pixel_renderer")))]
mod null_renderer;
#[cfg(feature = "pixel_renderer")]
mod pixel_renderer;
mod renderer;
#[cfg(feature = "svg_renderer")]
mod svg_renderer;

use anyhow::bail;

#[cfg(not(any(feature = "svg_renderer", feature = "pixel_renderer")))]
use crate::gcode_emulator::null_renderer::NullRenderer as MyRenderer;
#[cfg(feature = "pixel_renderer")]
use crate::gcode_emulator::pixel_renderer::PixelRenderer as MyRenderer;
#[cfg(feature = "svg_renderer")]
use crate::gcode_emulator::svg_renderer::SvgRenderer as MyRenderer;
use crate::{
    gcode_emulator::{
        Positioning::Relative,
        renderer::{RenderSettings, Renderer},
    },
    types::{
        coord::Coord,
        gcode::{GCode, GCodeOp},
        machine_settings::MachineState,
    },
};

enum Positioning {
    Relative,
    Absolute,
}

pub struct GCodeEmulator {
    gcode: GCode,
    state: MachineState,
    current_line: usize,
    positioning: Positioning,
    renderer: MyRenderer,
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
            positioning: Positioning::Absolute,
            renderer: MyRenderer::new(),
        })
    }

    pub fn step(&mut self) -> anyhow::Result<bool> {
        let next = loop {
            let next = match self.gcode.lines.get(self.current_line) {
                Some(line) => line,
                None => return Ok(false),
            };
            self.current_line += 1;
            if next.code.is_none() {
                continue;
            }
            break next;
        };

        // Safe to unwrap, we have already validated that next.code is not None.
        match next.code.as_ref().unwrap() {
            GCodeOp::G0 => {
                /* Move to X/Y rapid? */
                match next.coord {
                    None => {
                        bail!("G0 without a coord?!")
                    }
                    Some(coord) => {
                        self.renderer.draw_line(
                            self.state.pos,
                            coord,
                            RenderSettings {
                                color: "red".to_string(),
                                thickness: 0.1,
                                opacity: 0.5,
                            },
                        )?;
                        self.state.pos = coord;
                    }
                }
            }
            GCodeOp::G1 => {
                /* Move to X/Y, with feed and power */
                match next.coord {
                    None => {
                        bail!("G1 without a coord?!")
                    }
                    Some(coord) => {
                        self.renderer.draw_line(
                            self.state.pos,
                            coord,
                            RenderSettings {
                                color: "green".to_string(),
                                thickness: 0.1,
                                opacity: next.power.unwrap_or(0.0) / 1000f32,
                            },
                        )?;
                        self.state.pos = coord;
                    }
                }
            }
            GCodeOp::G21 => { /* Set units to mm */ }
            GCodeOp::G90 => self.positioning = Relative,
            GCodeOp::M4 => { /* Laser On */ }
            GCodeOp::M5 => { /* Laser Off */ }
            _ => bail!("Unknown code: {}", next),
        }

        Ok(true)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while self.step()? {}
        Ok(())
    }

    pub fn to_img_url(&mut self) -> anyhow::Result<String> {
        self.renderer.to_img_url()
    }

    pub fn save(&mut self, file: &str) -> anyhow::Result<()> {
        self.renderer.to_file(file)
    }
}

#[cfg(test)]
mod tests {
    use crate::gcode_emulator::GCodeEmulator;

    #[test]
    fn laser_gcode_test() {
        let mut gce = GCodeEmulator::from_file("../test_resources/test.gcode").unwrap();
        gce.run().unwrap();
        gce.save("out.svg").unwrap();
    }
}
