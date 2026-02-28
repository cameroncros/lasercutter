use anyhow::bail;
use cached::proc_macro::once;
use svg::{
    Document,
    node::element::{Group, Line, Path, path::Data},
};

use crate::{
    gcode_emulator::renderer::{RenderSettings, Renderer},
    types::coord::Coord,
};

#[once]
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

pub struct SvgRenderer {
    document: Option<Document>,
    last_line: Option<(Data, Coord, RenderSettings)>,
}

impl SvgRenderer {
    pub(crate) fn new() -> Self {
        let document = Document::new()
            .set("viewBox", (0, 0, 700, 700))
            .add(grid_layer());
        SvgRenderer {
            document: Some(document),
            last_line: None,
        }
    }
}

impl SvgRenderer {
    fn close_last(&mut self) -> anyhow::Result<()> {
        match self.last_line.take() {
            None => {}
            Some((line, _, last_render)) => {
                let path = Path::new()
                    .set("fill", true)
                    .set("fill-opacity", last_render.opacity)
                    .set("stroke", last_render.color)
                    .set("stroke-width", last_render.thickness)
                    .set("d", line);

                if let Some(doc) = self.document.take() {
                    self.document = Some(doc.add(path));
                } else {
                    bail!("No document set.")
                }
            }
        }
        Ok(())
    }
}

impl Renderer for SvgRenderer {
    fn draw_line(
        &mut self,
        start: Coord,
        end: Coord,
        render: RenderSettings,
    ) -> anyhow::Result<()> {
        match &self.last_line {
            None => {}
            Some((_, last_coord, last_render)) => {
                if *last_render != render || *last_coord != start {
                    self.close_last()?;
                }
            }
        }

        match self.last_line.take() {
            None => {
                let data = Data::new()
                    .move_to((start.0, start.1))
                    .line_to((end.0, end.1));
                let last = end;
                let last_render = render;
                self.last_line = Some((data, last, last_render))
            }
            Some((data, last, last_render)) => {
                let data = data.line_to((end.0, end.1));

                self.last_line = Some((data, last, last_render))
            }
        }
        Ok(())
    }

    fn to_svg_str(&mut self) -> anyhow::Result<String> {
        self.close_last()?;
        let svg_str = if let Some(document) = &self.document {
            document.to_string()
        } else {
            bail!("Document is not set");
        };
        Ok(svg_str)
    }

    fn to_file(&mut self, file: &str) -> anyhow::Result<()> {
        self.close_last()?;
        if let Some(document) = &self.document {
            svg::save(file, document)?;
        } else {
            bail!("Document is not set");
        }
        Ok(())
    }
}
