use std::sync::Mutex;

use anyhow::bail;
use svg::{
    Document,
    node::element::{Group, Line, Path, path::Data},
};

use crate::{
    gcode_emulator::renderer::{RenderSettings, Renderer},
    types::coord::Coord,
};

pub struct SvgRenderer {
    document: Mutex<Option<Document>>,
}

impl SvgRenderer {
    pub(crate) fn new() -> Self {
        let document = Document::new()
            .set("viewBox", (0, 0, 700, 700))
            .add(Self::grid_layer());
        SvgRenderer {
            document: Mutex::new(Some(document)),
        }
    }

    fn grid_layer() -> Group {
        const GRID_SIZE: i32 = 700;
        const MINOR_STEP: i32 = 10;
        const MAJOR_STEP: i32 = 50;

        let mut minor = Group::new()
            .set("stroke", "#e2e8f0")
            .set("stroke-width", 0.01)
            .set("opacity", 0.5)
            .set("shape-rendering", "crispEdges");
        let mut major = Group::new()
            .set("stroke", "#94a3b8")
            .set("stroke-width", 0.01)
            .set("opacity", 0.5)
            .set("shape-rendering", "crispEdges");

        let mut x = 0;
        while x <= GRID_SIZE {
            let line = Line::new()
                .set("x1", x)
                .set("y1", 0)
                .set("x2", x)
                .set("y2", GRID_SIZE);
            if x % MAJOR_STEP == 0 {
                major = major.add(line);
            } else {
                minor = minor.add(line);
            }
            x += MINOR_STEP;
        }

        let mut y = 0;
        while y <= GRID_SIZE {
            let line = Line::new()
                .set("x1", 0)
                .set("y1", y)
                .set("x2", GRID_SIZE)
                .set("y2", y);
            if y % MAJOR_STEP == 0 {
                major = major.add(line);
            } else {
                minor = minor.add(line);
            }
            y += MINOR_STEP;
        }

        Group::new().add(minor).add(major)
    }
}
impl Renderer for SvgRenderer {
    fn draw_line(
        &mut self,
        start: Coord,
        end: Coord,
        render: RenderSettings,
    ) -> anyhow::Result<()> {
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
            }
            Err(_) => bail!("Failed to lock document"),
        }
        Ok(())
    }

    fn to_svg_str(&self) -> anyhow::Result<String> {
        let svg_str = match self.document.lock() {
            Ok(dg) => {
                if let Some(document) = &*dg {
                    document.to_string()
                } else {
                    bail!("Document is not set");
                }
            }
            Err(_) => bail!("Failed to lock document"),
        };
        Ok(svg_str)
    }

    fn to_file(&mut self, file: &str) -> anyhow::Result<()> {
        match self.document.lock() {
            Ok(dg) => {
                if let Some(document) = &*dg {
                    svg::save(file, document)?;
                } else {
                    bail!("Document is not set");
                }
            }
            Err(_) => bail!("Failed to lock document"),
        }
        Ok(())
    }
}
