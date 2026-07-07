use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
};

use crate::types::coord::Coord;
#[derive(PartialEq, Debug)]
pub enum GCodeOp {
    G0,
    G1,
    G21,
    G90,
    M4,
    M5,
    UNKNOWN,
}

#[derive(PartialEq, Debug, Default)]
pub struct GCodeLine {
    pub(crate) code: Option<GCodeOp>,
    pub(crate) coord: Option<Coord>,
    pub(crate) power: Option<f32>,
    pub(crate) feedrate: Option<f32>,
    pub(crate) comment: Option<String>,
}

impl Display for GCodeLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = &self.code {
            match c {
                GCodeOp::G0 => f.write_str("G0 ")?,
                GCodeOp::G1 => f.write_str("G1 ")?,
                GCodeOp::G21 => f.write_str("G21 ")?,
                GCodeOp::G90 => f.write_str("G90 ")?,
                GCodeOp::M4 => f.write_str("M4 ")?,
                GCodeOp::M5 => f.write_str("M5 ")?,
                GCodeOp::UNKNOWN => f.write_str("UNKNOWN ")?,
            }
        }
        if let Some(coord) = self.coord {
            f.write_fmt(format_args!("X{} ", coord.0))?;
            f.write_fmt(format_args!("Y{} ", coord.1))?;
        }
        if let Some(power) = self.power {
            f.write_fmt(format_args!("S{} ", power))?;
        }
        if let Some(feedrate) = self.feedrate {
            f.write_fmt(format_args!("F{} ", feedrate))?;
        }

        f.write_str("; ")?;
        if let Some(comment) = &self.comment {
            f.write_str(comment)?;
        }
        Ok(())
    }
}

impl GCodeLine {
    fn from_str(args: &str) -> anyhow::Result<GCodeLine> {
        let mut x = None;
        let mut y = None;
        let mut s = None;
        let mut f = None;
        let mut code_str = "";

        for arg in args.split_ascii_whitespace() {
            match &arg.as_bytes()[0] {
                b'G' | b'g' | b'M' | b'm' => code_str = arg,
                b'X' | b'x' => x = Some(arg[1..].parse()?),
                b'Y' | b'y' => y = Some(arg[1..].parse()?),
                b'S' | b's' => s = Some(arg[1..].parse()?),
                b'F' | b'f' => f = Some(arg[1..].parse()?),
                b';' => break,
                _ => {}
            }
        }

        let coord = if let Some(x) = x
            && let Some(y) = y
        {
            Some(Coord(x, y))
        } else {
            None
        };

        let code = match code_str {
            "G0" | "g0" => Some(GCodeOp::G0),
            _ => None,
        };

        Ok(GCodeLine {
            code,
            coord,
            power: s,
            feedrate: f,
            comment: None,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct GCode {
    pub(crate) lines: Vec<GCodeLine>,
}

impl GCode {
    pub fn load(file: &str) -> anyhow::Result<GCode> {
        let lines = std::fs::read_to_string(file)?
            .lines()
            .map(GCodeLine::from_str)
            .collect::<anyhow::Result<Vec<GCodeLine>>>()?;
        Ok(GCode { lines })
    }

    pub fn save(&self, file: &str) -> anyhow::Result<()> {
        let mut f = File::create(file)?;
        for line in &self.lines {
            f.write_all(line.to_string().as_bytes())?;
            f.write_all(b"\n")?;
        }
        Ok(())
    }
}
