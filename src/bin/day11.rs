use aoc::get_lines;
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Monkey {
  items: Vec<u128>,
  operation: (String, String),
  test: u128,
  btree: [usize; 2],
}

#[derive(Default)]
struct Monkeys {
  pub data: Vec<Monkey>,
  pub total: Vec<u128>,
}

impl Monkeys {
  pub fn create(&mut self) {
    self.data.push(Monkey::default());
    self.total.push(0);
  }

  pub fn current(&mut self) -> &mut Monkey {
    let cm = self.data.len() - 1;
    self.data.get_mut(cm).unwrap()
  }

  pub fn solve<R>(&mut self, rounds: u128, relief: R) -> u128
  where
    R: Fn(u128) -> u128,
  {
    for _ in 0..rounds {
      for i in 0..self.data.len() {
        let wls: Vec<_> = self.data[i]
          .items
          .iter()
          .map(|item| {
            let val = match self.data[i].operation.0.as_str() {
              "old" => *item,
              _ => self.data[i].operation.0.parse::<u128>().unwrap(),
            };

            let mut wl = match self.data[i].operation.1.as_str() {
              "*" => *item * val,
              "+" => *item + val,
              _ => *item,
            };

            wl = relief(wl);

            wl
          })
          .collect();

        for wl in wls {
          if wl % self.data[i].test == 0 {
            let v = self.data[i].btree[0].clone();
            self.data[v].items.push(wl);
          } else {
            let v = self.data[i].btree[1].clone();
            self.data[v].items.push(wl);
          }
        }

        self.total[i] += self.data[i].items.len() as u128;
        self.data[i].items.clear();
      }
    }

    self.total.sort_by(|a, b| b.cmp(a));
    self.total[0] * self.total[1]
  }
}

fn main() {
  let mut monkeys = Monkeys::default();

  for data in get_lines("data/input-day11.txt") {
    let d: Vec<&str> = data.trim().split(':').collect();
    let c: Vec<&str> = d[0].split(' ').collect();

    match c[0] {
      "Monkey" => {
        monkeys.create();
      }
      "Starting" => {
        d[1].split(',').for_each(|item| {
          monkeys
            .current()
            .items
            .push(item.trim().parse::<u128>().unwrap())
        });
      }
      "Operation" => {
        let mut o: VecDeque<&str> = d[1].split(' ').collect();
        monkeys.current().operation =
          (o.pop_back().unwrap().into(), o.pop_back().unwrap().into());
      }
      "Test" => {
        monkeys.current().test = d[1].split(' ').last().unwrap().parse::<u128>().unwrap();
      }
      "If" => {
        let v = d[1].split(' ').last().unwrap().parse::<usize>().unwrap();
        match c[1] {
          "true" => monkeys.current().btree[0] = v,
          "false" => monkeys.current().btree[1] = v,
          _ => {}
        }
      }
      _ => {}
    }
  }

  println!("part 1: {}", monkeys.solve(20, |x| x / 3));

  let lcm = monkeys.data.iter().map(|m| m.test).product::<u128>();
  println!("part 2: {}", monkeys.solve(10000, |x| x % lcm));
}
