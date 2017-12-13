use std::io;
use std::io::Read;

fn severity(depths: &Vec<u32>, delay: u32, skip_on_caught: bool) -> (u32, bool) {
    let mut caught = false;
    let mut severity = 0;
    for i in 0..depths.len() {
        let depth = depths[i];
        if depth == 0 {
            continue
        }

        if (delay + i as u32) % ((depth - 1) * 2) == 0 {
            severity += depth * i as u32;
            caught = true;
            if skip_on_caught {
                return (0, caught);
            }
        }
    }
    return (severity, caught);
}

fn day13(input: &Vec<&str>) -> (u32, u32) {
    let mut depths: Vec<u32> = vec!();
    for line in input {
        let split = line.split(": ").collect::<Vec<&str>>();
        let next_index: usize = split[0].parse().unwrap();
        let next_depth: u32 = split[1].parse().unwrap();

        while next_index > depths.len() {
            let length = depths.len();
            depths.insert(length, 0);
        }
        depths.insert(next_index, next_depth);
    }

    let mut delay = 0;
    while severity(&depths, delay, true).1 {
        delay += 1;
    }

    return (severity(&depths, 0, false).0, delay);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let lines: Vec<&str> = buffer.lines().collect();

    let (part1, part2) = day13(&lines);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}