use crate::{
    gcode_emulator::renderer::{RenderSettings, Renderer},
    types::coord::Coord,
};

#[derive(Clone)]
pub struct NullRenderer {}

impl NullRenderer {
    pub fn new() -> Self {
        NullRenderer {}
    }
}

impl Renderer for NullRenderer {
    fn draw_line(
        &mut self,
        _start: Coord,
        _end: Coord,
        _render_settings: RenderSettings,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_file(&mut self, file: &str) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_img_url(&mut self) -> anyhow::Result<String> {
        Ok(String::from("Invalid Renderer"))
    }
}
