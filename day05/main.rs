use std::io;
use std::io::Read;

fn day5(initial: &Vec<i32>, f: fn(i32) -> i32) -> i32 {
    let mut input = initial.clone();
    let mut i: i32 = 0;
    let mut steps = 0;
    while i >= 0 && input.len() > i as usize {
        steps += 1;

        let jump = input[i as usize];
        input[i as usize] = f(jump);

        i += jump;
    }
    return steps;
}

fn part1(jump: i32) -> i32 {
    return jump + 1;
}

fn part2(jump: i32) -> i32 {
    return if jump >= 3 { jump - 1 } else { jump + 1 };
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let digits: Vec<i32> = buffer.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("part1: {}", day5(&digits, part1));
    println!("part2: {}", day5(&digits, part2));
}