use std::{error::Error, fs::File};
use std::result::Result;
use std::io::BufReader;

mod solvers;

#[allow(bare_trait_objects)]
fn main() -> Result<(), Box<Error>> {
    solvers::day01::solve(BufReader::new(File::open("input/input_01")?));
    solvers::day02::solve(BufReader::new(File::open("input/input_02")?));
    solvers::day03::solve(BufReader::new(File::open("input/input_03")?));
    solvers::day04::solve(BufReader::new(File::open("input/input_04")?)); // This one doesn't work yet
    solvers::day05::solve(BufReader::new(File::open("input/input_05")?));

    Ok(())
}