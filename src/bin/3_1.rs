const size: usize = 12;

fn main() {
    let mut freqs = [[0;size], [0; size]];
    let input = include_str!("../../inputs/3_1.txt");
    for line in input.lines() {
        for (idx, bit) in line.chars().enumerate() {
            match bit {
                '0' => freqs[0][idx] += 1,
                '1' => freqs[1][idx] += 1,
                _ => unreachable!()
            }
        }
    }
    println!("{:?}", freqs);
    let mut num: u32 = 0;
    for i in 0..size {
        if freqs[1][i] > freqs[0][i] { num += 0b1 };
        num = num << 1;
    }
    num = num >> 1;
    let other_num = !num & 0b00000000_00000000_00001111_11111111;
    println!("{}({:b}) & {}({:b})", num, num, other_num, other_num);
    println!("\nResult: {}", num * &other_num);
}
