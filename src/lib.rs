use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub enum Input {
    Puzzle,
    Test,
    TestB,
}

pub fn load_input(day: u8, part: Input) -> Vec<String> {
    load_input_iterator(day, part)
        .map(|line| line.expect("Couldn't read line"))
        .collect()
}

pub fn load_input_iterator(day: u8, part: Input) -> io::Lines<io::BufReader<fs::File>> {
    let suffix = match part {
        Input::Puzzle => "",
        Input::Test => "_test",
        Input::TestB => "_test_b",
    };
    load_file(Path::new(&format!("data/inputs/day{day}{suffix}.txt")))
}

fn load_file<P>(path: P) -> io::Lines<io::BufReader<fs::File>>
where
    P: AsRef<Path>,
{
    println!("Loading file {}", path.as_ref().display());
    let file = fs::File::open(path).expect("No file found at path");
    BufReader::new(file).lines()
}
