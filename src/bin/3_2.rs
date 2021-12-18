const size: usize = 12;

fn main() {
    let input = include_str!("../../inputs/3_1.txt");

    let mut ox_lines: Vec<&str> = input.lines().collect();
    let mut co2_lines: Vec<&str> = input.lines().collect();
    let mut idx = 0;
    while ox_lines.len() > 1 || co2_lines.len() > 1 {
        println!("[{}] {}:{}", idx, ox_lines.len(), co2_lines.len());
        if ox_lines.len() > 1 {
            let freqs = calc_freqs(&ox_lines);
            if freqs[1][idx] >= freqs[0][idx] {
                ox_lines = ox_lines.into_iter().filter(|&line| {line.as_bytes()[idx] == b'1'}).collect();
            } else {
                ox_lines = ox_lines.into_iter().filter(|&line| {line.as_bytes()[idx] == b'0'}).collect();
            }
        }
        if co2_lines.len() > 1 {
            let freqs = calc_freqs(&co2_lines);
            if freqs[1][idx] >= freqs[0][idx] {
                co2_lines = co2_lines.into_iter().filter(|&line| {line.as_bytes()[idx] == b'0'}).collect();
            } else {
                co2_lines = co2_lines.into_iter().filter(|&line| {line.as_bytes()[idx] == b'1'}).collect();
            }
        }
        idx += 1;
    }

    println!("{:?}\n{:?}", ox_lines, co2_lines);

    let ox_level = isize::from_str_radix(ox_lines[0], 2).unwrap();
    let co2_level = isize::from_str_radix(co2_lines[0], 2).unwrap();
    println!("Result: {} * {} = {}", ox_level, co2_level, ox_level * &co2_level);
}

fn calc_freqs(input: &Vec<&str>) -> [[i32; size]; 2] {
    let mut freqs: [[i32; size]; 2] = [[0;size];2];
    for line in input.into_iter() {
        for (idx, bit) in line.chars().enumerate() {
            match bit {
                '0' => freqs[0][idx] += 1,
                '1' => freqs[1][idx] += 1,
                _ => unreachable!()
            }
        }
    }
    freqs
}
