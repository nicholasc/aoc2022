use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let file = File::open("data/input-day1.txt").expect("Invalid file");
  let lines = io::BufReader::new(file).lines();

  let mut i = 0;
  let mut results = Vec::<u32>::new();

  for line in lines {
    if let Ok(data) = line {
      let num = data.as_str();
      match num {
        "" => {
          i += 1;
        }
        _ => {
          if i == results.len() {
            results.push(0);
          }

          results[i] += num.parse::<u32>().unwrap();
        }
      }
    }
  }

  // part 1
  if let Some(max) = results.iter().max() {
    println!("part 1: {}", max);
  }

  // part 2
  let mut top3 = 0;
  for _ in 0..3 {
    if let Some(max) = results.iter().max() {
      top3 += max;

      results.remove(results.iter().position(|&r| r == *max).unwrap());
    }
  }

  println!("part 2: {}", top3);
}
