use std::str;
use std::collections::HashSet;

struct UnionFind {
    uf: Vec<i32>,
    groups: usize,
}

impl UnionFind {
    fn create(capacity: usize) -> UnionFind {
        let mut uf: Vec<i32> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            uf.insert(i, i as i32);
        }
        UnionFind { uf, groups: capacity }
    }

    fn connect(&mut self, p: usize, q: usize) {
        let root1 = self.uf[p];
        let root2 = self.uf[q];

        if root1 != root2 {
            self.groups -= 1;
            for i in 0..self.uf.len() {
                if self.uf[i] == root1 {
                    self.uf[i] = root2;
                }
            }
        }
    }
}

fn day14(input: &&str) -> (u32, u32) {
    let capacity: u32 = 128 * 128;
    let mut uf = UnionFind::create(capacity as usize);

    let mut m: HashSet<i32> = HashSet::new();
    let mut sum = 0;
    for i in 0..128 {
        let row = format!("{}-{}", input, i);
        let hex = day10(256, &row);
        let mut col = 0;
        for h in hex.chars() {
            for c in format!("{:04b}", h.to_digit(16).unwrap()).chars() {
                let current = i * 128 + col as usize;
                match c {
                    '1' => {
                        sum += 1;
                        m.insert(current as i32);
                        let top_coordinate = (i as i32 - 1) * 128 + col;
                        let left_coordinate = i as i32 * 128 + col - 1;
                        if m.contains(&top_coordinate) {
                            uf.connect(current, top_coordinate as usize);
                        }
                        if left_coordinate % 128 != 127 && m.contains(&left_coordinate) {
                            uf.connect(current, left_coordinate as usize);
                        }
                    }
                    '0' => {}
                    _ => panic!(),
                }
                col += 1;
            }
        }
    }

    return (sum, uf.groups as u32 + sum - capacity);
}

fn day10_part1(size: usize, input: &Vec<u32>, rounds: u32) -> Vec<u32> {
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

fn day10_parse_input(input: &str) -> String {
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

fn day10_convert(input: &str) -> Vec<u32> {
    return input.lines().collect::<Vec<&str>>()[0]
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect();
}

fn day10(size: usize, input: &str) -> String {
    let array = day10_part1(size, &day10_convert(&day10_parse_input(input)), 64);
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
    let input = "amgozmfv";
//    let input = "flqrgnkx";

    let (part1, part2) = day14(&input);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}
