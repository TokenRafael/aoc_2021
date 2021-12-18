use std::ops::RangeInclusive;

const SIZE: usize = 1000;

fn mat_new(n: usize) -> Vec<i32> {
    vec![0; n * n]
}

fn get_interval(s: i32, f: i32) -> RangeInclusive<usize> {
    if f >= s {
        s as usize..=f as usize
    } else {
        f as usize..=s as usize
    }
}

fn get_double_interval(s: (i32, i32), f: (i32, i32)) -> Vec<(usize, usize)> {
    let ((s1, s2), (f1, f2)) = (s, f);
    let first_range: Vec<i32> = if f1 >= s1 {
        (s1..=f1).collect()
    } else {
        (f1..=s1).rev().collect()
    };
    let second_range: Vec<i32> = if f2 >= s2 {
        (s2..=f2).collect()
    } else {
        (f2..=s2).rev().collect()
    };
    first_range
        .into_iter()
        .zip(second_range.into_iter())
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}

fn trace_map(map: &mut Vec<i32>, line: ((i32, i32), (i32, i32))) {
    match line {
        ((row_s, col_s), (row_f, col_f)) if col_f == col_s => {
            let col = col_s;
            for row in get_interval(row_s, row_f){
                let idx = SIZE * row + col as usize;
                map[idx] += 1;
            }

        },
        ((row_s, col_s), (row_f, col_f)) if row_f == row_s => {
            let row = row_s;
            for col in get_interval(col_s,col_f) {
                let idx = SIZE * (row as usize) + col;
                map[idx] += 1;
            }

        },
        (s, f)=> {
            let interval = get_double_interval(s, f);
            // println!("{:?} -> {:?} = {:?}", s, f, interval);

            for (row, col) in interval {
                let idx = SIZE * (row) + col;
                map[idx] += 1;
            }
        }
    }
}

fn line_from_string(s: &str) -> ((i32, i32), (i32, i32)) {
    let (start, finish) = s.split_once(" -> ").unwrap();
    let (start_tuple, finish_tuple) = (
        start.split_once(",").unwrap(),
        finish.split_once(",").unwrap(),
    );
    (
      (start_tuple.0.parse().unwrap(), start_tuple.1.parse().unwrap()),
      (finish_tuple.0.parse().unwrap(), finish_tuple.1.parse().unwrap()),
    )
}

fn main() {
    let mut map = mat_new(SIZE);
    let input = include_str!("../../inputs/5_1.txt");
    for line in input.lines() {
      let line = line_from_string(line);
      trace_map(&mut map, line);
    }

    let count = map.into_iter().filter(|&val| val > 1).count();
    println!("Result: {}", count);
}
