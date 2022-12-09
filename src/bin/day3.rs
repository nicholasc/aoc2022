use aoc::get_lines;
use std::cmp::Ordering;

fn get_priority(c: char) -> u32 {
  let code = c as u32;
  match code.cmp(&97) {
    Ordering::Less => code - 38,
    _ => code - 96,
  }
}

fn main() {
  let mut totals: [u32; 2] = [0, 0];
  let mut group = Vec::<String>::new();

  for line in get_lines("data/input-day3.txt") {
    if let Ok(data) = line {
      // part 1
      let (c1, c2) = data.split_at((data.len() / 2) as usize);
      for c1c in c1.chars().into_iter() {
        if let Some(_) = c2.chars().into_iter().find(|&c_| c_ == c1c) {
          totals[0] += get_priority(c1c);
          break;
        }
      }

      // part 2
      group.push(data);
      if group.len() == 3 {
        for c1c in group[0].chars().into_iter() {
          if let Some(_) = group[1].chars().into_iter().find(|&c_| c_ == c1c) {
            if let Some(_) = group[2].chars().into_iter().find(|&c_| c_ == c1c)
            {
              totals[1] += get_priority(c1c);
              break;
            }
          }
        }
        group.clear();
      }
    }
  }

  println!("part 1: {}", totals[0]);
  println!("part 2: {}", totals[1]);
}
