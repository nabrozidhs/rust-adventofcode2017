use std::io;
use std::io::Read;
use std::collections::HashMap;

fn comp_fn(f: &&str) -> fn(i32, i32) -> bool {
    return match *f {
        "==" => |a, b| a == b,
        "<=" => |a, b| a <= b,
        ">=" => |a, b| a >= b,
        ">" => |a, b| a > b,
        "<" => |a, b| a < b,
        "!=" => |a, b| a != b,
        _ => panic!(""),
    };
}

fn op_fn(f: &&str) -> fn(i32, i32) -> i32 {
    return match *f {
        "inc" => |a, b| a + b,
        "dec" => |a, b| a - b,
        _ => panic!(""),
    };
}

fn day8(input: &Vec<&str>) -> (i32, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut highest_value = 0;
    for line in input {
        let s: Vec<&str> = line.split_whitespace().collect();
        let to_check = *registers.get(&s[4]).unwrap_or(&0);
        if comp_fn(&s[5])(to_check, s[6].parse().unwrap()) {
            let old_value = *registers.get(&s[0]).unwrap_or(&0);
            let new_value = op_fn(&s[1])(old_value, s[2].parse().unwrap());
            if new_value > highest_value {
                highest_value = new_value;
            }
            registers.insert(s[0], new_value);
        }
    }
    return (*registers.iter()
        .max_by(|l, r| l.1.cmp(r.1))
        .map(|e| e.1)
        .unwrap(), highest_value);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<&str> = buffer.lines().collect();

    let (part1, part2) = day8(&input);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}