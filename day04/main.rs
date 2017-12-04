use std::io;
use std::io::Read;
use std::collections::HashSet;

fn day4(lines: &Vec<Vec<&str>>, f: fn(&&str) -> String) -> u32 {
    return lines.iter()
        .filter(|line| {
            let mut set: HashSet<String> = HashSet::new();
            for word in line.iter().map(f) {
                let added = set.insert(word);
                if !added {
                    return false;
                }
            }
            return true;
        })
        .count() as u32;
}

fn part1(input: &&str) -> String {
    return input.to_string();
}

fn part2(input: &&str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    return chars.iter().collect();
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let digits: Vec<Vec<&str>> = buffer.lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    println!("part1 {}", day4(&digits, part1));
    println!("part2 {}", day4(&digits, part2));
}