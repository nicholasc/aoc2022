use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn get_lines(file: &str) -> Lines<BufReader<File>> {
  let file = File::open(file).expect("Invalid file");
  io::BufReader::new(file).lines()
}
