use std::io;
use std::io::Read;

fn day12(input: &Vec<&str>) -> (u32, u32) {
    let mut uf: Vec<u32> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        uf.insert(i, i as u32);
    }
    let mut groups: u32 = input.len() as u32;

    for line in input {
        let parsed = line.split(" <-> ").collect::<Vec<&str>>();
        let p: u32 = parsed[0].parse().unwrap();
        let numbers_to_connect = parsed[1].split(", ").map(|e| e.parse().unwrap()).collect::<Vec<u32>>();

        for num in numbers_to_connect {
            let root1 = uf[p as usize];
            let root2 = uf[num as usize];

            if root1 == root2 {
                continue
            }

            groups -= 1;
            for i in 0..uf.len() {
                if uf[i] == root1 {
                    uf[i] = root2;
                }
            }
        }
    }

    let mut same = 0;
    for i in 0..uf.len() {
        if uf[i] == uf[0] {
            same += 1;
        }
    }
    return (same, groups);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<&str> = buffer.lines().collect();

    let (part1, part2) = day12(&input);
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}