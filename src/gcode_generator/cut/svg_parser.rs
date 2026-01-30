use crate::gcode_generator::cut::{Cut, Segment};
use crate::types::Coord;
use anyhow::bail;
use svg::node::element::path::{Command, Data};
use svg::node::{Attributes, Value};
use svg::node::element::tag::Type;
use svg::parser::Event;

struct Transform {
    rotate: (f32, f32, f32, f32),
    offset: Coord,
}

impl Transform {
    fn from(transform: &Value) -> anyhow::Result<Self> {
        let str = transform.to_string();
        let t = str.replace("matrix(", "")
            .replace(")", "")
            .replace(",", " ")
            .split_whitespace().map(|s| s.parse::<f32>()).collect::<Result<Vec<_>, _>>()
            .map(|v| Self { rotate: (v[0], v[1], v[2], v[3]), offset: Coord(v[4], v[5]) })?;
        Ok(t)
    }

    fn apply(&self, coord: Coord) -> Coord {
        let Coord(x, y) = coord;
        let Coord(ox, oy) = self.offset;
        Coord((self.rotate.0 * x + self.rotate.2 * x) + ox,
              (self.rotate.1 * y + self.rotate.3 * y) + oy)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self { rotate: (1.0, 0.0, 0.0, -1.0), offset: Coord::default() }
    }
}

impl Cut {
    fn from_svg_path(attributes: &Attributes, transform: &Transform) -> anyhow::Result<Vec<Segment>> {
        let mut segments = vec![];
        let Some(data) = attributes.get("d") else {
            bail!("Missing path data");
        };
        let data = Data::parse(data)?;
        let mut position = Coord::default();
        for command in data.iter() {
            match command {
                Command::Move(_pos, params) => {
                    position = transform.apply(Coord(params[0], params[1]));
                },
                Command::Line(_pos, params) => {
                    for next_pos in params.chunks(2) {
                        match next_pos {
                            &[next_x, next_y] => {
                                let next = transform.apply(Coord(next_x, next_y));
                                segments.push(Segment::Line(position, next));
                                position = next;
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                Command::Close => {
                    let Some(last) = segments.last() else {
                        bail!("Missing last segment");
                    };
                    let Some(first) = segments.first() else {
                        bail!("Missing first segment");
                    };
                    let first_pos = match first {
                        Segment::Line(f, _) => {f}
                        Segment::Curve => {unimplemented!()}
                    };
                    let last_pos = match last {
                        Segment::Line(_, l) => {l}
                        Segment::Curve => {unimplemented!()}
                    };
                    segments.push(Segment::Line(*last_pos, *first_pos));
                }
                e => { bail!("Unknown command: {e:?}"); }
            }
        }
        Ok(segments)
    }

    pub fn from_svg(file_path: &str) -> anyhow::Result<Cut> {
        let mut content = String::new();
        let mut cut = Cut {
            origin: Coord::default(),
            rotation: 0.0,
            cuts: vec![],
        };

        let mut transform = Transform::default();
        for event in svg::open(file_path, &mut content)? {
            match event {
                Event::Tag(name, t, attributes) => {
                    if name == "path" {
                        let segments = Self::from_svg_path(&attributes, &transform)?;
                        cut.cuts.extend(segments);
                    }
                    if name == "g" {
                        match t {
                            Type::Start => {
                                if let Some(s) = attributes.get("transform") {
                                    transform = Transform::from(s)?;
                                }
                            }
                            Type::End => {
                                transform = Transform::default();
                            }
                            Type::Empty => {}
                        }

                    }
                }
                _ => {}
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