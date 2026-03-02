use crate::types::coord::Coord;

#[derive(PartialEq)]
pub struct RenderSettings {
    pub color: String,
    pub thickness: f32,
    pub opacity: f32,
}

pub trait Renderer {
    fn draw_line(&mut self, start: Coord, end: Coord, render: RenderSettings)
    -> anyhow::Result<()>;

    fn to_file(&mut self, file: &str) -> anyhow::Result<()>;

    fn to_img_url(&mut self) -> anyhow::Result<String>;
}
