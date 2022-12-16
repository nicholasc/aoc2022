use aoc::get_lines;

fn main() {
  let mut x = 1;
  let mut total = 0;
  let mut cycles = 1;
  let mut crt = Vec::<Vec<char>>::new();

  let mut cycle = |x: &mut i32, a: Option<i32>| {
    if (cycles - 20) % 40 == 0 {
      total += cycles * *x;
    }

    let p = cycles - 1;
    let r = (p / 40) as usize;
    if crt.len() == r {
      crt.push(Vec::<char>::new())
    }

    if p >= *x - 1 + (r as i32 * 40) && p <= *x + 1 + (r as i32 * 40) {
      crt[r].push('#');
    } else {
      crt[r].push('.');
    }

    cycles += 1;
    if let Some(v) = a {
      *x += v;
    }
  };

  for data in get_lines("data/input-day10.txt") {
    let d: Vec<&str> = data.split(' ').collect();

    match d[0] {
      "addx" => {
        cycle(&mut x, None);
        cycle(&mut x, Some(d[1].parse::<i32>().unwrap()));
      }
      "noop" => {
        cycle(&mut x, None);
      }
      _ => {}
    }
  }

  println!("part 1: {}", total);
  println!("part 2:");
  for r in crt.iter() {
    for c in r.iter() {
      print!("{}", c);
    }
    println!();
  }
}
