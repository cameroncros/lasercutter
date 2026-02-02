use std::{fs::File, io::Write};

pub struct GCode {
    pub(crate) lines: Vec<String>,
}

impl GCode {
    pub fn load(file: &str) -> anyhow::Result<GCode> {
        let lines = std::fs::read_to_string(file)?
            .lines()
            .map(|l| l.to_string())
            .collect();
        Ok(GCode { lines })
    }

    pub fn save(&self, file: &str) -> anyhow::Result<()> {
        let mut f = File::create(file)?;
        for line in &self.lines {
            f.write_all(line.as_bytes())?;
            f.write_all(b"\n")?;
        }
        Ok(())
    }
}
