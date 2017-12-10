use std::io;
use std::io::Read;
use std::str;

fn part1(size: usize, input: &Vec<u32>, rounds: u32) -> Vec<u32> {
    let mut array: Vec<u32> = Vec::with_capacity(size);
    for i in 0..size {
        array.insert(i, i as u32);
    }

    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..rounds {
        for sublist_size in input {
            for i in 0..(sublist_size / 2) {
                let c = (current_position + i) as usize % array.len();
                let n = (current_position + sublist_size - i - 1) as usize % array.len();
                let temp = array[c];
                array[c] = array[n];
                array[n] = temp;
            }
            current_position = (current_position + sublist_size + skip_size) % array.len() as u32;
            skip_size += 1;
        }
    }

    return array;
}

fn day10_part1(size: usize, input: &str) -> u32 {
    let array = part1(size, &convert(input), 1);
    return array[0] * array[1];
}

fn parse_part2_input(input: &str) -> String {
    let mut output: Vec<String> = vec!();

    for c in input.chars() {
        let size = output.len();
        output.insert(size, (c as u8).to_string());
    }

    if output.is_empty() {
        String::from("17,31,73,47,23")
    } else {
        output.join(",") + ",17,31,73,47,23"
    }
}

fn convert(input: &str) -> Vec<u32> {
    return input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect();
}

fn day10_part2(size: usize, input: &str) -> String {
    let array = part1(size, &convert(&parse_part2_input(input)), 64);
    let mut hex: String = String::from("");
    for i in 0..16 {
        let mut c: u8 = 0;
        for k in 0..16 {
            c ^= array[i * 16 + k] as u8;
        }
        hex = hex + &format!("{:02x}", c);
    }

    return hex;
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1 {}", day10_part1(256, &buffer));
    println!("part2 {}", day10_part2(256, &buffer));
}