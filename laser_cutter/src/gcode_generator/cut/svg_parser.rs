use std::{f32::consts::PI, path::PathBuf};

use anyhow::{Context, bail};
use nalgebra::{Matrix2, Matrix2x1};
use svg::{
    node::{
        Attributes,
        element::{
            path::{
                Command,
                Data,
                Position,
                Position::{Absolute, Relative},
            },
            tag::Type,
        },
    },
    parser::Event,
};

use crate::{
    gcode_generator::cut::{Cut, Segment},
    types::{coord::Coord, transform::Transform},
};

impl Cut {
    fn from_svg_rect(
        attributes: &Attributes,
        transform: &Transform,
    ) -> anyhow::Result<Vec<Segment>> {
        // Future: Use transform.
        let Some(x) = attributes.get("x") else {
            bail!("Missing path data");
        };
        let Some(y) = attributes.get("y") else {
            bail!("Missing path data");
        };
        let Some(h) = attributes.get("height") else {
            bail!("Missing path data");
        };
        let Some(w) = attributes.get("width") else {
            bail!("Missing path data");
        };
        let x = x.parse::<f32>().context("Failed to parse x as f32")?;
        let y = y.parse::<f32>().context("Failed to parse y as f32")?;
        let h = h.parse::<f32>().context("Failed to parse h as f32")?;
        let w = w.parse::<f32>().context("Failed to parse w as f32")?;
        let segments = vec![
            Segment::Line(Coord(x, y), Coord(x + w, y)).transform(transform),
            Segment::Line(Coord(x + w, y), Coord(x + w, y + h)).transform(transform),
            Segment::Line(Coord(x + w, y + h), Coord(x, y + h)).transform(transform),
            Segment::Line(Coord(x, y + h), Coord(x, y)).transform(transform),
        ];

        Ok(segments)
    }

    fn from_svg_path(
        attributes: &Attributes,
        transform: &Transform,
    ) -> anyhow::Result<Vec<Segment>> {
        let mut segments = vec![];
        let Some(data) = attributes.get("d") else {
            bail!("Missing path data");
        };
        let data = Data::parse(data)?;
        let mut position = Coord::default();
        for command in data.iter() {
            match command {
                Command::Move(pos, params) => {
                    let coords = params
                        .chunks(2)
                        .map(|f| Coord(f[0], f[1]))
                        .collect::<Vec<Coord>>();
                    let first = coords.first().context("Missing first coordinate")?;
                    match pos {
                        Absolute => position = *first,
                        Relative => position += *first,
                    }
                    for next in coords[1..].iter() {
                        match pos {
                            Relative => {
                                segments.push(Segment::Line(position, position + *next));
                                position += *next;
                            }
                            Absolute => {
                                segments.push(Segment::Line(position, *next));
                                position = *next;
                            }
                        }
                    }
                }
                Command::Line(pos, params) => {
                    let coords = params
                        .chunks(2)
                        .map(|f| Coord(f[0], f[1]))
                        .collect::<Vec<Coord>>();
                    for next in coords {
                        match pos {
                            Relative => {
                                segments.push(Segment::Line(position, position + next));
                                position += next;
                            }
                            Absolute => {
                                segments.push(Segment::Line(position, next));
                                position = next;
                            }
                        }
                    }
                }
                Command::HorizontalLine(pos, params) => {
                    for param in params.iter() {
                        let next = match pos {
                            Position::Absolute => Coord(*param, position.1),
                            Relative => Coord(position.0 + *param, position.1),
                        };
                        segments.push(Segment::Line(position, next));
                        position = next;
                    }
                }
                Command::VerticalLine(pos, params) => {
                    for param in params.iter() {
                        let next = match pos {
                            Position::Absolute => Coord(position.0, *param),
                            Relative => Coord(position.0, position.1 + *param),
                        };
                        segments.push(Segment::Line(position, next));
                        position = next;
                    }
                }
                Command::Close => {
                    let Some(last) = segments.last() else {
                        bail!("Missing last segment");
                    };
                    let Some(first) = segments.first() else {
                        bail!("Missing first segment");
                    };
                    let first_pos = match first {
                        Segment::Line(f, _) => f,
                        Segment::Curve => {
                            unimplemented!()
                        }
                    };
                    let last_pos = match last {
                        Segment::Line(_, l) => l,
                        Segment::Curve => {
                            unimplemented!()
                        }
                    };
                    segments.push(Segment::Line(*last_pos, *first_pos));
                }
                Command::EllipticalArc(pos, parameters) => {
                    for next_pos in parameters.chunks(7) {
                        println!("EllipticalArc:{pos:?}:{next_pos:?}");
                        let radius = Coord(next_pos[0], next_pos[1]);
                        let rotation = next_pos[2];
                        let large_arc = next_pos[3] != 0.0;
                        let sweep = next_pos[4] != 0.0;
                        let end = match pos {
                            Absolute => Coord(next_pos[5], next_pos[6]),
                            Relative => position + Coord(next_pos[5], next_pos[6]),
                        };
                        let new_segments = interpolate_elliptical_arc(
                            position, end, radius, rotation, large_arc, sweep,
                        )?;
                        segments.extend(new_segments);
                        position = end;
                    }
                }
                e => {
                    bail!("Unknown command: {e:?}");
                }
            }
        }

        // Apply transform
        segments = segments.iter().map(|s| s.transform(transform)).collect();
        Ok(segments)
    }

    pub fn from_svg(file_path: PathBuf) -> anyhow::Result<Cut> {
        let mut content = String::new();
        let mut cut = Cut {
            source: Some(file_path.clone()),
            transform: Transform::default(),
            cuts: vec![],
        };

        let mut transform = Transform::default();
        for event in svg::open(&file_path, &mut content)? {
            if let Event::Tag(name, t, attributes) = event {
                match name {
                    "path" => {
                        let segments = Self::from_svg_path(&attributes, &transform)?;
                        cut.cuts.extend(segments);
                    }
                    "rect" => {
                        let segments = Self::from_svg_rect(&attributes, &transform)?;
                        cut.cuts.extend(segments);
                    }
                    "g" => match t {
                        Type::Start => {
                            if let Some(s) = attributes.get("transform") {
                                transform = Transform::try_from(s)?;
                            }
                        }
                        Type::End => {
                            transform = Transform::default();
                        }
                        Type::Empty => {}
                    },
                    a => {
                        println!("Unknown tag: {a}");
                    }
                }
            }
        }

        Ok(cut)
    }
}

fn square(v: f32) -> f32 {
    v * v
}

fn angle(a: Coord, b: Coord) -> f32 {
    let first = (a * b / (a.dist() * a.dist())).acos();
    let sign = a.0 * b.1 - a.1 - b.0;
    first.copysign(sign)
}

// https://svg-tutorial.com/editor/arc
// https://www.w3.org/TR/SVG/implnote.html#ArcImplementationNotes
fn interpolate_elliptical_arc(
    start: Coord,
    end: Coord,
    radius: Coord,
    phi: f32,
    large_arc: bool,
    sweep: bool,
) -> anyhow::Result<Vec<Segment>> {
    // Step 1: Compute (x1′, y1′)
    let xy_prime = Coord::from(
        Matrix2::new(phi.cos(), phi.sin(), -phi.sin(), phi.cos())
            * Matrix2x1::new((start.0 - end.0) / 2.0, (start.1 - end.1) / 2.0),
    );
    // Step 2: Compute (cx′, cy′)
    let det = (square(radius.0) * square(radius.1)
        - square(radius.0) * square(xy_prime.1)
        - square(radius.1) * square(xy_prime.0))
        / (square(radius.0) * square(xy_prime.1) + square(radius.1) * square(xy_prime.0));
    let c_prime = det.sqrt()
        * Coord(
            (radius.0 * xy_prime.1) / radius.1,
            -(radius.1 * xy_prime.0) / radius.0,
        );
    let c_prime = if large_arc == sweep {
        -1.0 * c_prime
    } else {
        c_prime
    };
    // Step 3: Compute (cx, cy) from (cx′, cy′)
    let c = (Matrix2::new(phi.cos(), -phi.sin(), phi.sin(), phi.cos())
        * Matrix2x1::new(c_prime.0, c_prime.1))
        + Matrix2x1::new((start.0 + end.0) / 2.0, (start.1 + end.1) / 2.0);
    let c = Coord(c.x, c.y);

    // Step 4: Compute θ1 and Δθ
    let theta_1 = angle(
        Coord(1.0, 0.0),
        Coord(
            (xy_prime.0 - c_prime.0) / radius.0,
            (xy_prime.1 - c_prime.1) / radius.1,
        ),
    );
    let theta_delta = angle(
        Coord(
            (xy_prime.0 - c_prime.0) / radius.0,
            (xy_prime.1 - c_prime.1) / radius.1,
        ),
        Coord(
            (-xy_prime.0 - c_prime.0) / radius.0,
            (-xy_prime.1 - c_prime.1) / radius.1,
        ),
    ) % (2.0 * PI);

    let theta_delta = if sweep == false && theta_delta > 0.0 {
        theta_delta - (2.0 * PI)
    } else if sweep == true && theta_delta < 0.0 {
        theta_delta + (2.0 * PI)
    } else {
        theta_delta
    };

    let mut segments = vec![];

    let num_steps = (theta_delta * 180.0 / PI).abs().ceil();
    let step_size = theta_delta / num_steps;

    let mut last = Coord(
        c.0 + radius.0 * theta_1.cos(),
        c.1 + radius.1 * theta_1.sin(),
    );

    for step in 1..(num_steps as i32) {
        let next_a = theta_1 + (step as f32) * step_size;
        let next_coord = Coord(c.0 + radius.0 * next_a.cos(), c.1 + radius.1 * next_a.sin());

        segments.push(Segment::Line(last, next_coord));
        last = next_coord;
    }

    let last_a = theta_1 + theta_delta;
    let very_last = Coord(c.0 + radius.0 * last_a.cos(), c.1 + radius.1 * last_a.sin());
    segments.push(Segment::Line(last, very_last));

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use crate::{
        gcode_emulator::GCodeEmulator,
        gcode_generator::{cut::Cut, workspace::Workspace},
    };

    #[test]
    fn test_cut_from_svg() {
        Cut::from_svg("../test_resources/box-all/input.svg".into()).unwrap();
    }

    #[test]
    fn test_cut_from_elliptical() {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg("../test_resources/test_cases/input.svg".into()).unwrap());
        let gcode = w.gen_gcode().unwrap();
        let mut emu = GCodeEmulator::from_gcode(gcode).unwrap();
        emu.run().unwrap();
        emu.save("output.svg").unwrap();
    }

    #[test]
    fn test_cut_from_ellipticalq() {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg("../test_resources/arcs01/input.svg".into()).unwrap());
        let gcode = w.gen_gcode().unwrap();
        let mut emu = GCodeEmulator::from_gcode(gcode).unwrap();
        emu.run().unwrap();
        emu.save("output.svg").unwrap();
    }
}
