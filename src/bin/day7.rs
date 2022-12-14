use aoc::get_lines;
use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug)]
struct Directory {
  name: String,
  dirs: Vec<Directory>,
  files: HashMap<String, u32>,
}

impl Directory {
  pub fn size(&self) -> u32 {
    let mut total = 0;
    for (_, size) in self.files.iter() {
      total += size;
    }

    for dir in self.dirs.iter() {
      total += dir.size();
    }

    total
  }

  pub fn part1(&self, total: &mut u32) {
    let size = self.size();
    if size <= 100000 {
      *total += size;
    }

    for dir in self.dirs.iter() {
      dir.part1(total);
    }
  }

  pub fn part2(&self, space: u32, total: &mut u32) {
    let size = self.size();
    if size > 30000000 - space && size < *total {
      *total = size;
    }

    for dir in self.dirs.iter() {
      dir.part2(space, total);
    }
  }
}

fn get_cwd<'a>(
  fs: &'a mut Vec<Directory>,
  cpath: &VecDeque<String>,
) -> &'a mut Directory {
  let mut cwd = &mut fs[0];
  for dir in cpath.iter() {
    let i = cwd.dirs.iter().position(|d| d.name == *dir).unwrap();
    cwd = &mut cwd.dirs[i];
  }
  cwd
}

fn main() {
  let mut totals: [u32; 2] = [0, 30000000];
  let mut cpath = VecDeque::<String>::new();
  let mut fs = Vec::<Directory>::new();
  let mut ls = false;

  for data in get_lines("data/input-day7.txt") {
    let tokens: Vec<&str> = data.split(' ').collect();

    match tokens[0] {
      "$" => match tokens[1] {
        "cd" => {
          if ls {
            ls = false;
          }
          let folder = tokens[2].into();
          if folder == ".." {
            cpath.pop_back();
          } else if folder == "/" {
            fs.push(Directory {
              name: folder,
              ..Default::default()
            });
          } else {
            let cwd = get_cwd(&mut fs, &cpath);
            match cwd.dirs.iter().find(|&d| d.name == folder).is_some() {
              true => {
                cpath.push_back(folder);
              }
              false => {
                if folder != "/" {
                  cpath.push_back(folder);
                }
                cwd.dirs.push(Directory {
                  name: tokens[2].into(),
                  ..Default::default()
                });
              }
            }
          }
        }
        "ls" => ls = true,
        _ => {}
      },
      "dir" => {
        if ls {
          let folder = tokens[1].into();
          let cwd = get_cwd(&mut fs, &cpath);

          match cwd.dirs.iter().find(|&d| d.name == folder).is_some() {
            false => {
              cwd.dirs.push(Directory {
                name: folder,
                ..Default::default()
              });
            }
            _ => {}
          }
        }
      }
      _ => {
        if ls {
          get_cwd(&mut fs, &cpath)
            .files
            .insert(tokens[1].into(), tokens[0].parse::<u32>().unwrap());
        }
      }
    }
  }

  fs[0].part1(&mut totals[0]);
  fs[0].part2(70000000 - fs[0].size(), &mut totals[1]);

  println!("part 1: {}", totals[0]);
  println!("part 2: {}", totals[1]);
}
