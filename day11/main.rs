use std::io;
use std::io::Read;

#[derive(Debug)]
struct Hex {
    x: i32,
    y: i32,
}

fn move_to(hex: &Hex, next: &str) -> Hex {
    return match next {
        "n" => Hex { x: hex.x, y: hex.y - 1 },
        "s" => Hex { x: hex.x, y: hex.y + 1 },
        "ne" => Hex { x: hex.x + 1, y: hex.y - (hex.x % 2).abs() },
        "se" => Hex { x: hex.x + 1, y: hex.y + if hex.x % 2 == 0 { 1 } else { 0 } },
        "nw" => Hex { x: hex.x - 1, y: hex.y - (hex.x % 2).abs() },
        "sw" => Hex { x: hex.x - 1, y: hex.y + if hex.x % 2 == 0 { 1 } else { 0 } },
        _ => panic!(""),
    };
}

fn distance(start: &Hex, end: &Hex) -> i32 {
    let distance_x = (start.x - end.x).abs();
    let distance_y = (start.y - end.y).abs();

    if end.y > start.y {
        distance_x + distance_y - distance_x / 2 - if distance_x % 2 == 0 { 0 } else { 1 }
    } else {
        distance_x + distance_y - distance_x / 2
    }
}

fn day11(input: &Vec<&str>) -> (i32, i32) {
    let start = Hex { x: 0, y: 0 };
    let mut hex = Hex { x: 0, y: 0 };
    let mut max_distance = 0;
    for next in input {
        hex = move_to(&hex, next);
        let next_distance = distance(&start, &hex);
        max_distance = if next_distance > max_distance { next_distance } else { max_distance };
    }
    (distance(&start, &hex), max_distance)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<&str> = buffer.lines().collect::<Vec<&str>>()[0]
        .split(",").collect::<Vec<&str>>();

    let (part1, part2) = day11(&input);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}