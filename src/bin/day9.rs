use aoc::get_lines;
use std::collections::HashMap;

fn handle_rope(size: usize) -> usize {
  let mut r = vec![(0, 0); size];
  let mut paths = HashMap::<String, bool>::new();
  paths.insert("0,0".into(), true);

  for data in get_lines("data/input-day9.txt") {
    let d: Vec<&str> = data.split(' ').collect();
    let c = d[1].parse::<u32>().unwrap();

    let f = match d[0] {
      "L" => (-1, 0),
      "R" => (1, 0),
      "U" => (0, 1),
      _ => (0, -1),
    };

    for _ in 0..c {
      r[0].0 += f.0;
      r[0].1 += f.1;

      for i in 1..r.len() {
        let xd: i32 = r[i - 1].0 - r[i].0;
        let yd: i32 = r[i - 1].1 - r[i].1;

        if xd.abs() <= 1 && yd.abs() <= 1 {
          continue;
        }

        let tf = match (xd, yd) {
          (x, y) if x == 0 && y > 0 => (0, 1),
          (x, y) if x == 0 && y < 0 => (0, -1),
          (x, y) if y == 0 && x > 0 => (1, 0),
          (x, y) if y == 0 && x < 0 => (-1, 0),
          (x, y) if x > 0 && y > 0 => (1, 1),
          (x, y) if x < 0 && y > 0 => (-1, 1),
          (x, y) if x > 0 && y < 0 => (1, -1),
          (x, y) if x < 0 && y < 0 => (-1, -1),
          _ => (0, 0),
        };

        r[i].0 += tf.0;
        r[i].1 += tf.1;

        if i == r.len() - 1 {
          let p = r[i].0.to_string() + "," + &r[i].1.to_string();
          if !paths.contains_key(&p) {
            paths.insert(p, true);
          }
        }
      }
    }
  }

  paths.len()
}

fn main() {
  println!("part 1: {}", handle_rope(2));
  println!("part 2: {}", handle_rope(10));
}
