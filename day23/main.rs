use std::io;
use std::io::Read;
use std::collections::HashMap;

fn day23(input: &Vec<&str>) -> i64 {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut idx = 0;
    let mut mul_called = 0;
    while idx < input.len() {
        let cmd = input[idx].split_whitespace().collect::<Vec<&str>>();
        match cmd[0] {
            "set" => {
                let value: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), value);
                idx += 1;
            }
            "sub" => {
                let a: i64 = *registers.get(cmd[1]).unwrap_or(&0);
                let b: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), a - b);
                idx += 1;
            }
            "mul" => {
                let a: i64 = *registers.get(cmd[1]).unwrap_or(&0);
                let b: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), a * b);
                idx += 1;
                mul_called += 1;
            }
            "jnz" => {
                if cmd[1].parse().unwrap_or(*registers.get(cmd[1]).unwrap_or(&0)) != 0 {
                    idx = (idx as i64 + cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0))) as usize;
                } else {
                    idx += 1;
                }
            }
            _ => panic!(),
        }
    }

    mul_called
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1: {}", day23(&buffer.lines().collect()));
    println!("Second part is just iterating from 106500 to 123500 (both inclusive) and count the non prime numbers.");
}