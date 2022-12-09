use aoc::get_lines;

fn main() {
  let mut totals: [u32; 2] = [0, 0];

  for line in get_lines("data/input-day4.txt") {
    if let Ok(data) = line {
      let ranges: Vec<Vec<u32>> = data
        .split(',')
        .map(|r| r.split('-').map(|v| v.parse::<u32>().unwrap()).collect())
        .collect();

      // part 1
      match (&ranges[0], &ranges[1]) {
        (a, b) if a[0] >= b[0] && a[1] <= b[1] => totals[0] += 1,
        (a, b) if b[0] >= a[0] && b[1] <= a[1] => totals[0] += 1,
        _ => {}
      }

      // part 2
      match (&ranges[0], &ranges[1]) {
        (a, b) if b[0] >= a[0] && b[0] <= a[1] => totals[1] += 1,
        (a, b) if a[0] >= b[0] && a[0] <= b[1] => totals[1] += 1,
        (a, b) if b[1] >= a[0] && b[1] <= a[1] => totals[1] += 1,
        (a, b) if a[1] >= b[0] && a[1] <= b[1] => totals[1] += 1,
        _ => {}
      }
    }
  }

  println!("part 1: {}", totals[0]);
  println!("part 2: {}", totals[1]);
}
