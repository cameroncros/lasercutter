use crate::types::coord::Coord;

pub struct RenderSettings {
    pub color: String,
    pub thickness: f32,
    pub opacity: f32,
}

pub trait Renderer {
    fn draw_line(&mut self, start: Coord, end: Coord, render: RenderSettings)
    -> anyhow::Result<()>;
    fn to_svg_str(&self) -> anyhow::Result<String>;

    fn to_file(&mut self, file: &str) -> anyhow::Result<()>;
}
