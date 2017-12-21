use std::io;
use std::io::Read;
use std::collections::HashMap;

fn flip(input: &Vec<bool>) -> Vec<bool> {
    let mut flipped: Vec<bool> = vec!();

    let size = ((input.len() as f64).sqrt()) as usize;
    for row in 0..size {
        for col in 0..size {
            flipped.push(input[row * size + (size - 1 - col)]);
        }
    }

    flipped
}

fn rotate(input: &Vec<bool>) -> Vec<bool> {
    let mut rotated: Vec<bool> = vec!();

    let size = ((input.len() as f64).sqrt()) as usize;
    for row in 0..size {
        for col in 0..size {
            rotated.push(input[(size - col - 1) * size + row]);
        }
    }

    rotated
}

fn parse_array(input: &str) -> Vec<bool> {
    let mut output: Vec<bool> = vec!();
    for c in input.chars() {
        match c {
            '#' => output.push(true),
            '.' => output.push(false),
            _ => {}
        }
    }
    output
}

fn parse_input(input: &Vec<&str>) -> HashMap<Vec<bool>, Vec<bool>> {
    let mut m: HashMap<Vec<bool>, Vec<bool>> = HashMap::new();

    for line in input {
        let split = line.split(" => ").collect::<Vec<&str>>();
        let output_array = parse_array(&split[1]);
        let input1 = parse_array(&split[0]);
        let input2 = rotate(&input1);
        let input3 = rotate(&input2);
        let input4 = rotate(&input3);

        for a in vec!(input1, input2, input3, input4) {
            m.insert(a.clone(), output_array.clone());
            m.insert(flip(&a.clone()), output_array.clone());
        }
    }

    m
}

fn step(state: &Vec<bool>, m: &HashMap<Vec<bool>, Vec<bool>>) -> Vec<bool> {
    let size = ((state.len() as f64).sqrt()) as usize;
//    let section_size = if size % 3 == 0 { 3 } else { 2 };
    let section_size = if size % 2 == 0 { 2 } else { 3 };
    let sections_per_row = size / section_size;
    let n_sections = sections_per_row * sections_per_row;

    let mut sections: Vec<Vec<bool>> = vec!();
    for i in 0..n_sections {
        let mut section: Vec<bool> = vec!();

        for row in 0..section_size {
            let from_row = row + section_size * (i / sections_per_row);
            for col in 0..section_size {
                let from_col = col + section_size * (i % sections_per_row);
                section.push(state[from_row * size + from_col])
            }
        }

        sections.push(section);
    }

    let new_section_size = section_size + 1;
    let new_size = sections_per_row * new_section_size;
    let mut new_state = Vec::new();
    for _ in 0..(new_size * new_size) { new_state.push(false); }

    for i in 0..sections.len() {
        let output = m.get(&sections[i]).unwrap();
        for row in 0..new_section_size {
            let to_row = row + new_section_size * (i / sections_per_row);
            for col in 0..new_section_size {
                let to_col = col + new_section_size * (i % sections_per_row);
                new_state[to_row * new_size + to_col] = output[row * new_section_size + col];
            }
        }
    }

    new_state
}

fn day21(input: &Vec<&str>, iterations: u32) -> u64 {
    let m = parse_input(&input);

    let mut current = parse_array(".#./..#/###");

    for _ in 0..iterations {
        current = step(&current, &m);
    }

    current.iter().filter(|b| **b == true).count() as u64
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1: {}", day21(&buffer.lines().collect(), 5));
    println!("part2: {}", day21(&buffer.lines().collect(), 18));
}