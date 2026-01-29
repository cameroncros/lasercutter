use crate::gcode_emulator::renderer::{RenderSettings, Renderer};
use crate::types::Coord;
use anyhow::bail;
use std::sync::Mutex;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub struct SvgRenderer {
    document: Mutex<Option<Document>>
}

impl SvgRenderer {
    pub(crate) fn new() -> Self {
        let document = Document::new()
            .set("viewBox", (0, 0, 700, 700));
        SvgRenderer {
            document: Mutex::new(Some(document))
        }
    }
}
impl Renderer for SvgRenderer {
    fn draw_line(&mut self, start: Coord, end: Coord, render: RenderSettings) -> anyhow::Result<()> {
        match self.document.lock() {
            Ok(mut dg) => {
                if let Some(document) = dg.take() {
                    let data = Data::new()
                        .move_to((start.0, start.1))
                        .line_to((end.0, end.1))
                        .close();

                    let path = Path::new()
                        .set("fill", true)
                        .set("fill-opacity", render.opacity)
                        .set("stroke", render.color)
                        .set("stroke-width", render.thickness)
                        .set("d", data);
                    let document = document.add(path);
                    *dg = Some(document);
                } else {
                    bail!("Document is not set");
                }
            },
            Err(_) => bail!("Failed to lock document")
        }
        Ok(())
    }

    fn save(&mut self, file: &str) -> anyhow::Result<()> {
        match self.document.lock() {
            Ok(dg) => {
                if let Some(document) = &*dg {
                    svg::save(file, document)?;
                } else {
                    bail!("Document is not set");
                }
            },
            Err(_) => bail!("Failed to lock document")
        }
        Ok(())
    }
}