use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(file: &str) -> Vec<String> {
  let file = File::open(file).expect("Invalid file");
  io::BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .collect::<Vec<String>>()
}
