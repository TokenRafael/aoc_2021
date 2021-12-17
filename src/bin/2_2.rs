use std::{str::FromStr, num::ParseIntError};

struct Sub {
  hor: i32,
  depth: i32,
  aim: i32,
}

impl Sub {
  fn new() -> Self {
    Sub { hor: 0, depth: 0, aim: 0 }
  }

  fn movement(&mut self, m: Move) {
    use Move::{Fwd, Down, Up};
    match m {
      Fwd(x) => {
        self.hor += x;
        self.depth += self.aim * x;
      },
      Down(x) => self.aim += x,
      Up(x) => self.aim -= x,
    };
  }
}

enum Move {
  Fwd(i32),
  Down(i32),
  Up(i32)
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::{Fwd, Down, Up};

        let (cmd, val) = s.split_once(" ").unwrap();

        match cmd {
            "forward" => Ok(Fwd(val.parse()?)),
            "down" => Ok(Down(val.parse()?)),
            "up" => Ok(Up(val.parse()?)),
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut sub = Sub::new();
    let input = include_str!("../../inputs/2_1.txt");
    for line in input.lines() {
        let cmd = Move::from_str(line).unwrap();
        sub.movement(cmd);
    }
    println!("{}", sub.hor * sub.depth);
}
