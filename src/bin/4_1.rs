use std::{num::ParseIntError, process::exit, str::FromStr};

const SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Cell {
    drawn: bool,
    val: i32,
}

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn mark(&mut self, num: i32) {
        if let Some(idx) = self.cells.iter_mut().position(|cell| cell.val == num) {
            self.cells[idx] = Cell{ drawn: true, ..self.cells[idx] }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..SIZE {

            if self
                .cells
                .iter()
                .skip(i * SIZE)
                .take(SIZE)
                .filter_map(|cell| if cell.drawn { Some(1) } else { None })
                .sum::<i32>() as usize
                == SIZE
                || self
                    .cells
                    .iter()
                    .skip(i)
                    .step_by(SIZE)
                    .take(SIZE)
                    .filter_map(|cell| if cell.drawn { Some(1) } else { None })
                    .sum::<i32>() as usize
                    == SIZE
            {
                return true;
            }
        }

        false
    }

    fn points(&self, last_val: i32) -> i32 {
        let sum: i32 = self
            .cells
            .iter()
            .filter_map(|c| if !c.drawn { Some(c.val) } else { None })
            .sum();
        sum * last_val
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut vals = Vec::with_capacity(SIZE);
        for row in s.split("\n") {
            let row = row.trim();

            for num in row.split_whitespace() {
                let val: i32 = num.parse()?;
                vals.push(Cell{ val, drawn: false });
            }
        }
        Ok(Board { cells: vals })
    }
}

fn main() {
    let input = include_str!("../../inputs/4_1.txt");
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers: Vec<i32> = numbers
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut boards: Vec<Board> = boards
        .split("\n\n")
        .map(Board::from_str)
        .map(Result::unwrap)
        .collect();

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.has_won() {
                println!("Winner found with score: {}", board.points(number));
                exit(0);
            }
        }
    }
}
