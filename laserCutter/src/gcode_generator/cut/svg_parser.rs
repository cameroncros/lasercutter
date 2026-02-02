use anyhow::{Context, bail};
use svg::{
    node::{
        Attributes,
        Value,
        element::{
            path::{Command, Data},
            tag::Type,
        },
    },
    parser::Event,
};

use crate::{
    gcode_generator::cut::{Cut, Segment},
    types::{coord::Coord, transform::Transform},
};

impl TryFrom<&Value> for Transform {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let str = value.to_string();
        let t = str
            .replace("matrix(", "")
            .replace(")", "")
            .replace(",", " ")
            .split_whitespace()
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Transform {
                rotate: (v[0], v[1], v[2], v[3]),
                offset: Coord(v[4], v[5]),
            })?;
        Ok(t)
    }
}

impl Cut {
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
                Command::Move(_pos, params) => {
                    let coords = params
                        .chunks(2)
                        .map(|f| Coord(f[0], f[1]))
                        .collect::<Vec<Coord>>();
                    let first = coords.first().context("Missing first coordinate")?;
                    position = transform.apply(first);
                    for next_pos in coords[1..].iter() {
                        let next = transform.apply(&next_pos);
                        segments.push(Segment::Line(position, next));
                        position = next;
                    }
                }
                Command::Line(_pos, params) => {
                    let coords = params
                        .chunks(2)
                        .map(|f| Coord(f[0], f[1]))
                        .collect::<Vec<Coord>>();
                    for next_pos in coords {
                        let next = transform.apply(&next_pos);
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
                e => {
                    bail!("Unknown command: {e:?}");
                }
            }
        }
        Ok(segments)
    }

    pub fn from_svg(file_path: &str) -> anyhow::Result<Cut> {
        let mut content = String::new();
        let mut cut = Cut {
            source: Some(file_path.to_string()),
            transform: Transform::default(),
            cuts: vec![],
        };

        let mut transform = Transform::default();
        for event in svg::open(file_path, &mut content)? {
            if let Event::Tag(name, t, attributes) = event {
                if name == "path" {
                    let segments = Self::from_svg_path(&attributes, &transform)?;
                    cut.cuts.extend(segments);
                }
                if name == "g" {
                    match t {
                        Type::Start => {
                            if let Some(s) = attributes.get("transform") {
                                transform = Transform::try_from(s)?;
                            }
                        }
                        Type::End => {
                            transform = Transform::default();
                        }
                        Type::Empty => {}
                    }
                }
            }
        }

        Ok(cut)
    }
}

#[cfg(test)]
mod tests {
    use crate::gcode_generator::cut::Cut;

    #[test]
    fn test_cut_from_svg() {
        Cut::from_svg("resources/box-all.svg").unwrap();
    }
}
