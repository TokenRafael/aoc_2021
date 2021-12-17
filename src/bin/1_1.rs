use std::str;

fn main() {
    let input = include_str!("../../inputs/1_1.txt");
    let measurements = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    let mut sum = 0;
    for m in measurements.windows(2).peekable() {
        if m.get(1).is_none() { break }
        if m[1] - m[0] > 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
}
