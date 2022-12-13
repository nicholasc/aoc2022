use aoc::get_lines;

fn start_of(c: char, b: &mut Vec<char>, l: usize) -> bool {
  b.push(c);

  let mut bc = b.clone();
  bc.sort();
  bc.dedup();

  if b.len() == l {
    b.remove(0);
  }

  bc.len() == l
}

fn main() {
  let mut sopb: bool = false;
  let mut sop = Vec::<char>::new();

  let mut somb: bool = false;
  let mut som = Vec::<char>::new();

  let lines = get_lines("data/input-day6.txt");

  for (i, c) in lines.first().unwrap().chars().enumerate() {
    if !sopb && start_of(c, &mut sop, 4) {
      println!("part 1: {}", i + 1);
      sopb = true;
    }

    if !somb && start_of(c, &mut som, 14) {
      println!("part 2: {}", i + 1);
      somb = true;
    }
  }
}
