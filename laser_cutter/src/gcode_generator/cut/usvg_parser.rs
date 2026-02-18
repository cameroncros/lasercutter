use std::{fs, path::PathBuf};

use anyhow::bail;
use usvg::{
    Node,
    Path,
    tiny_skia_path::PathSegment::{Close, CubicTo, LineTo, MoveTo, QuadTo},
};

use crate::{
    gcode_generator::cut::{Cut, Segment},
    types::{
        coord::{Coord, midpoint},
        transform::Transform,
    },
};

impl Cut {
    fn from_svg_path(path: &Path) -> anyhow::Result<Vec<Segment>> {
        let mut segments = vec![];
        let data = path.data().segments();
        let mut position = Coord::default();
        for command in data {
            match command {
                MoveTo(point) => {
                    let next = Coord(point.x, point.y);
                    position = next;
                }
                LineTo(point) => {
                    let next = Coord(point.x, point.y);
                    segments.push(Segment::Line(position, next));
                    position = next;
                }
                QuadTo(control, end) => {
                    let control = Coord(control.x, control.y);
                    let end = Coord(end.x, end.y);
                    let start = position;
                    for i in 0..100 {
                        let ratio = i as f32 / 100.0;
                        let q0 = midpoint(&start, &control, ratio);
                        let q1 = midpoint(&control, &end, ratio);
                        let next = midpoint(&q0, &q1, ratio);

                        segments.push(Segment::Line(position, next));
                        position = next;
                    }
                }
                CubicTo(control1, control2, end) => {
                    let start = position;

                    let control1 = Coord(control1.x, control1.y);
                    let control2 = Coord(control2.x, control2.y);
                    let end = Coord(end.x, end.y);
                    for i in 0..100 {
                        let ratio = i as f32 / 100.0;
                        let q0 = midpoint(&start, &control1, ratio);
                        let q1 = midpoint(&control1, &control2, ratio);
                        let q2 = midpoint(&control2, &end, ratio);

                        let r0 = midpoint(&q0, &q1, ratio);
                        let r1 = midpoint(&q1, &q2, ratio);

                        let next = midpoint(&r0, &r1, ratio);

                        segments.push(Segment::Line(position, next));
                        position = next;
                    }
                }
                Close => {
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
            }
        }

        // Apply transform
        let transform = Transform::from(&path.abs_transform());

        segments = segments.iter().map(|s| s.transform(&transform)).collect();
        Ok(segments)
    }

    pub fn from_svg(file_path: PathBuf) -> anyhow::Result<Cut> {
        let mut cut = Cut {
            source: Some(file_path.clone()),
            transform: Transform::default(),
            cuts: vec![],
        };

        let content = fs::read_to_string(&file_path)?;
        let tree = usvg::Tree::from_str(&content, &usvg::Options::default())?;

        fn all_nodes(parent: &usvg::Group) -> Vec<&Path> {
            let mut found_nodes = vec![];
            for node in parent.children() {
                match node {
                    Node::Group(g) => {
                        found_nodes.extend(all_nodes(g));
                    }
                    Node::Path(p) => found_nodes.push(p),
                    Node::Image(_) => {
                        println!("Images not handled");
                    }
                    Node::Text(_) => {
                        println!("Text not handled");
                    }
                }
            }

            found_nodes
        }

        let all_paths = all_nodes(tree.root());
        for path in all_paths {
            cut.cuts.extend(Self::from_svg_path(path)?);
        }

        Ok(cut)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use crate::{
        gcode_emulator::GCodeEmulator,
        gcode_generator::{cut::Cut, workspace::Workspace},
    };

    #[test]
    fn test_from_svg() {
        let cut = Cut::from_svg(PathBuf::from("../test_resources/arcs01/input.svg")).unwrap();
    }

    #[test_case("box-all")]
    #[test_case("test_cases")]
    #[test_case("elip-arc")]
    #[test_case("float-issue")]
    #[test_case("arcs01")]
    fn test_cut_from_elliptical(test: &str) {
        let mut w = Workspace::init(1000.0, 1000.0);
        w.add_cut(Cut::from_svg(format!("../test_resources/{test}/input.svg").into()).unwrap());
        let gcode = w.gen_gcode().unwrap();
        let mut emu = GCodeEmulator::from_gcode(gcode).unwrap();
        emu.run().unwrap();
        emu.save(&format!("../test_resources/{test}/output.svg"))
            .unwrap();
    }
}
