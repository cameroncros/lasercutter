use anyhow::bail;
use svg::node::element::path::{Command, Data};
use svg::parser::Event;
use crate::gcode_generator::cut::{Cut, Segments};
use crate::types::Coord;

impl Cut {
    pub fn from_svg(file_path: &str) -> anyhow::Result<Cut> {
        let mut content = String::new();
        let mut cut = Cut {
            origin: Coord::default(),
            rotation: 0.0,
            cuts: vec![],
        };

        for event in svg::open(file_path, &mut content)? {
            match event {
                Event::Tag(_, _, attributes) => {
                    let Some(data) = attributes.get("d") else {
                        continue;
                    };
                    let data = Data::parse(data)?;
                    let mut position = Coord::default();
                    for command in data.iter() {
                        match command {
                            Command::Move(_pos, params) => {
                                position = Coord(params[0], params[1]);
                            },
                            Command::Line(_pos, params) => {
                                for next_pos in params.chunks(2) {
                                    match next_pos {
                                        &[next_x, next_y] => {
                                            let next = Coord(next_x, next_y);
                                            cut.cuts.push(Segments::Line(position, next));
                                            position = next;
                                        },
                                        _ => unreachable!(),
                                    }
                                }
                            },
                            Command::Close => {}
                            e => { bail!("Unknown command: {e:?}"); }
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