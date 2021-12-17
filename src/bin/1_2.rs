use std::str;

fn main() {
    let input = include_str!("../../inputs/1_1.txt");
    let measurements: Vec<i32> = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let cascading: Vec<i32> = measurements
        .windows(3)
        .map(|m| m.iter().sum())
        .collect();

    let mut sum = 0;
    for m in cascading.windows(2).peekable() {
        if m.get(1).is_none() {
            break;
        }
        if m[1] - m[0] > 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
}
