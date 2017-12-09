use std::io;
use std::io::Read;

fn day9(input: &Vec<char>) -> (u32, u32) {
    let mut score = 0;
    let mut stack_size = 0;
    let mut garbage = 0;

    let mut ignore = false;
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            '{' if !ignore => {
                stack_size += 1;
                score += stack_size;
            }
            '}' if !ignore => stack_size -= 1,
            '<' if !ignore => ignore = true,
            '>' if ignore => ignore = false,
            '!' => i += 1,
            _ if ignore => garbage += 1,
            _ => {}
        }
        i += 1;
    }

    return (score, garbage);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<char> = buffer.lines().collect::<Vec<&str>>()[0].chars().collect();

    let (part1, part2) = day9(&input);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}