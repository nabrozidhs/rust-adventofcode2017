use std::io;
use std::io::Read;

fn day2(spreadsheet: &[Vec<u32>], f: fn(Vec<u32>) -> u32) -> u32 {
    return spreadsheet
        .iter()
        .map(|row| {
            let mut newrow = row.to_vec();
            newrow.sort();
            return f(newrow);
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();
}

fn part1(sorted_row: Vec<u32>) -> u32 {
    return sorted_row[sorted_row.len() - 1] - sorted_row[0];
}

fn part2(sorted_row: Vec<u32>) -> u32 {
    for i in 0..sorted_row.len() {
        for j in i + 1..sorted_row.len() {
            if sorted_row[j] % sorted_row[i] == 0 {
                return sorted_row[j] / sorted_row[i];
            }
        }
    }
    panic!("couldn't find divisible items")
}

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(_) => { panic!("") }
    }

    let digits: Vec<Vec<u32>> = buffer.lines()
        .map(|line| line.split_whitespace().map(|element| element.parse().unwrap()).collect())
        .collect();

    println!("part1 {}", day2(&digits, part1));
    println!("part2 {}", day2(&digits, part2));
}