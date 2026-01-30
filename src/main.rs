use clap::Parser;
use crate::gcode_emulator::GCodeEmulator;
use crate::gcode_generator::cut::Cut;
use crate::gcode_generator::workspace::Workspace;

#[allow(dead_code)]
mod gcode_emulator;

#[allow(dead_code)]
mod gcode_generator;
mod types;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Args {
    SvgToGcode {
        #[clap(short, long)]
        input_file: String,
        #[clap(short, long)]
        output_file: String,
    },
    GcodeToSvg {
        #[clap(short, long)]
        input_file: String,
        #[clap(short, long)]
        output_file: String,
    }
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();
    match args {
        Args::SvgToGcode { input_file, output_file } => {
            let mut w = Workspace::init(100.0, 100.0);
            w.add_cut(Cut::from_svg(&input_file)?);
            w.gen_gcode()?.save(&output_file)
        }
        Args::GcodeToSvg { input_file, output_file } => {
            let mut gce = GCodeEmulator::load(&input_file)?;
            gce.run()?;
            gce.save(&output_file)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gcode_emulator::GCodeEmulator;
    use crate::gcode_generator::cut::Cut;
    use crate::gcode_generator::workspace::Workspace;

    #[test]
    fn end_to_end_test() {
        let mut w = Workspace::init(100.0, 100.0);
        w.add_cut(Cut::from_svg("resources/box-all.svg").unwrap());
        w.gen_gcode().unwrap().save("out.gcode").unwrap();

        let mut gce = GCodeEmulator::load("out.gcode").unwrap();
        gce.run().unwrap();
        gce.save("out.svg").unwrap()
    }
}
