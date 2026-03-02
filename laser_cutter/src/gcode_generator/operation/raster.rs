use std::{fmt::Display, io::Cursor, path::PathBuf};

use image::{GenericImageView, ImageReader, Rgba};
use serde::{Deserialize, Serialize};

use crate::{
    gcode_generator::operation::{
        Line,
        Operation,
        OperationTrait,
        deserialize_cuts,
        serialize_cuts,
    },
    types::{coord::Coord, transform::Transform},
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Raster {
    pub source: Option<PathBuf>,
    pub transform: Transform,
    #[serde(
        serialize_with = "serialize_cuts",
        deserialize_with = "deserialize_cuts"
    )]
    pub cuts: Vec<Line>,
}

impl Display for Raster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.source {
            Some(ref source) => write!(
                f,
                "Raster: {}",
                source.file_name().unwrap().to_string_lossy()
            ),
            None => write!(f, "Cut: Unknown"),
        }
    }
}

impl OperationTrait for Raster {
    fn bounds(&self) -> (Coord, Coord) {
        let mut min = Coord(f32::MAX, f32::MAX);
        let mut max = Coord(f32::MIN, f32::MIN);
        for Line(start, end) in self.cuts.iter() {
            let start = self.transform.apply(start);
            let end = self.transform.apply(end);
            min.0 = min.0.min(start.0);
            min.0 = min.0.min(end.0);
            min.1 = min.1.min(start.1);
            min.1 = min.1.min(end.1);
            max.0 = max.0.max(start.0);
            max.0 = max.0.max(end.0);
            max.1 = max.1.max(start.1);
            max.1 = max.1.max(end.1);
        }
        (min, max)
    }
}

impl Raster {
    pub fn from_image(image_path: PathBuf) -> anyhow::Result<Operation> {
        let bytes = std::fs::read(&image_path)?;

        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        let (h, w) = img.dimensions();

        let step = Coord(0.0, 0.1);
        let newline_step = Coord(0.1, 0.0);

        let mut cuts = vec![];

        fn coord_to_pixel(coord: Coord) -> (i32, i32) {
            (coord.0 as i32, coord.1 as i32)
        }

        let out_of_bounds =
            |x: i32, y: i32| -> bool { x < 0 || x > (w - 2) as i32 || y < 0 || y > (h - 2) as i32 };

        let mut current = Coord(0.0, 0.0);

        let mut move_to = |delta: Coord| -> bool {
            let next = current + delta;
            let (x, y) = coord_to_pixel(next);
            let ofb = out_of_bounds(x, y);
            let _pixel = if !ofb {
                img.get_pixel(x as u32, y as u32)
            } else {
                Rgba([0, 0, 0, 0])
            };
            cuts.push(Line(current, next));
            current = next;
            ofb
        };

        let mut reverse = false;
        loop {
            let out_of_bound = move_to(if !reverse {
                step
            } else {
                Coord(0.0, 0.0) - step
            });

            if out_of_bound {
                move_to(newline_step);
                reverse = !reverse;

                let out_of_bound = move_to(if !reverse {
                    step
                } else {
                    Coord(0.0, 0.0) - step
                });

                if out_of_bound {
                    break;
                }
            }
        }

        let raster = Raster {
            source: Some(image_path),
            transform: Default::default(),
            cuts,
        };

        Ok(Operation::Raster(raster))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        gcode_emulator::GCodeEmulator,
        gcode_generator::{operation::raster::Raster, workspace::Workspace},
    };

    #[test]
    fn test_raster() {
        let mut workspace = Workspace::init(700.0, 700.0);
        let raster = Raster::from_image("../test_resources/g2.png".into()).unwrap();
        workspace.add_operation(raster);
        let gcode = workspace.gen_gcode().unwrap();
        let mut emu = GCodeEmulator::from_gcode(gcode).unwrap();
        emu.run().unwrap();
    }
}
