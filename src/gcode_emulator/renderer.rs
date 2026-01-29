use crate::types::Coord;

pub struct RenderSettings {
    pub color: String,
    pub thickness: f32,
    pub opacity: f32,
}

pub trait Renderer {
    fn draw_line(&mut self, start: Coord, end: Coord, render: RenderSettings) -> anyhow::Result<()>;

    fn save(&mut self, file: &str) -> anyhow::Result<()>;
}