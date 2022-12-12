use aoc::get_lines;
use std::collections::VecDeque;

fn main() {
  let mut stacks: [[VecDeque<char>; 9]; 2] = Default::default();

  for (i, line) in get_lines("data/input-day5.txt").enumerate() {
    if let Ok(data) = line {
      match i {
        i if i < 8 => {
          data
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(i, c)| {
              if !c[1].eq(&' ') {
                stacks[0][i].push_back(c[1]);
                stacks[1][i].push_back(c[1]);
              }
            });
        }
        i if i > 9 => {
          let c: Vec<usize> = data
            .split(' ')
            .map(|c| c.parse::<usize>().unwrap_or(0))
            .collect();

          let mut tmp = VecDeque::<char>::new();
          for _ in 0..c[1] {
            // part 1
            let d = stacks[0][c[3] - 1].pop_front().unwrap().clone();
            stacks[0][c[5] - 1].push_front(d);

            // part 2
            let d = stacks[1][c[3] - 1].pop_front().unwrap().clone();
            tmp.push_front(d);
          }

          tmp.iter().for_each(|s| stacks[1][c[5] - 1].push_front(*s));
        }
        _ => {}
      }
    }
  }

  let mut results: [String; 2] = Default::default();
  for i in 0..stacks.len() {
    results[i] = stacks[i]
      .clone()
      .map(|mut s| s.pop_front().unwrap())
      .iter()
      .collect();
  }

  println!("part 1: {}", results[0]);
  println!("part 2: {}", results[1]);
}
