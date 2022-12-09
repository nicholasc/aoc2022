use aoc::get_lines;

fn main() {
  let mut rounds = Vec::<[u32; 2]>::new();

  for line in get_lines("data/input-day2.txt") {
    if let Ok(data) = line {
      let vec: Vec<&str> = data.split(" ").collect();

      match vec[0] {
        "A" => match vec[1] {
          "X" => rounds.push([4, 3]),
          "Y" => rounds.push([8, 4]),
          "Z" => rounds.push([3, 8]),
          _ => panic!("Invalid input"),
        },
        "B" => match vec[1] {
          "X" => rounds.push([1, 1]),
          "Y" => rounds.push([5, 5]),
          "Z" => rounds.push([9, 9]),
          _ => panic!("Invalid input"),
        },
        "C" => match vec[1] {
          "X" => rounds.push([7, 2]),
          "Y" => rounds.push([2, 6]),
          "Z" => rounds.push([6, 7]),
          _ => panic!("Invalid input"),
        },
        _ => panic!("Invalid input"),
      };
    }
  }

  let totals = rounds
    .iter()
    .copied()
    .reduce(|acc, item| [&acc[0] + &item[0], &acc[1] + &item[1]])
    .unwrap();

  println!("part 1: {}", totals[0]);
  println!("part 2: {}", totals[1]);
}
