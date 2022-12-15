use aoc::get_lines;

fn is_edge(r: usize, c: usize, g: &Vec<Vec<u32>>) -> bool {
  r == 0 || r == g.len() - 1 || c == 0 || c == g.len() - 1
}

fn get_visibility(r: usize, c: usize, g: &Vec<Vec<u32>>, d: (i32, i32)) -> (bool, u32) {
  let v = g[r][c];
  let mut dist = 1;
  let mut ret = (true, 0);
  let mut x = c as i32 + d.1;
  let mut y = r as i32 + d.0;

  loop {
    let c = g[y as usize][x as usize];
    if c >= v {
      ret = (false, dist);
      break;
    }

    if is_edge(y as usize, x as usize, g) {
      ret = (ret.0, dist);
      break;
    }

    x += d.1;
    y += d.0;
    dist += 1;
  }

  ret
}

fn update_score(t: u32, b: u32, l: u32, r: u32, s: &mut u32) {
  let score = t * b * l * r;
  if *s < score {
    *s = score;
  }
}

fn main() {
  let mut totals: [u32; 2] = [0, 0];
  let mut g: Vec<Vec<u32>> = Default::default();

  for d in get_lines("data/input-day8.txt") {
    g.push(d.chars().map(|c| c.to_digit(10).unwrap()).collect());
  }

  for (r, row) in g.iter().enumerate() {
    for (c, _) in row.iter().enumerate() {
      if is_edge(r, c, &g) {
        totals[0] += 1;
        continue;
      }

      match (
        get_visibility(r, c, &g, (-1, 0)),
        get_visibility(r, c, &g, (1, 0)),
        get_visibility(r, c, &g, (0, -1)),
        get_visibility(r, c, &g, (0, 1)),
      ) {
        (t, b, l, r) if t.0 || b.0 || l.0 || r.0 => {
          totals[0] += 1;
          update_score(t.1, b.1, l.1, r.1, &mut totals[1]);
        }
        (t, b, l, r) => update_score(t.1, b.1, l.1, r.1, &mut totals[1]),
      }
    }
  }

  println!("part 1: {}", totals[0]);
  println!("part 2: {}", totals[1]);
}
