mod error;
mod solver;

use std::process::ExitCode;

use clap::Parser;

use crate::solver::possibilities;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub sequence: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let possibilities = match possibilities(args.sequence.as_str()) {
        Ok(possibilities) => possibilities,
        Err(why) => {
            eprintln!("{why}");
            return ExitCode::FAILURE;
        }
    };

    for item in possibilities {
        println!("{item}");
    }

    ExitCode::SUCCESS
}
