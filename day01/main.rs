use std::io;
use std::io::Read;

fn day1(digits: &Vec<u32>, jump: usize) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..digits.len() {
        let next = (i + jump) % digits.len();

        if digits[i] == digits[next] {
            sum += digits[i]
        }
    }
    return sum;
}

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(_) => { panic!("") }
    }

    let digits: Vec<u32> = buffer.split("\n").collect::<Vec<&str>>()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // part1
    println!("part1 {}", day1(&digits, 1));
    // part2
    println!("part2 {}", day1(&digits, digits.len() / 2));
}