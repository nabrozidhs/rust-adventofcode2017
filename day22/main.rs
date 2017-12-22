use std::io;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
enum State {
    CLEAN,
    WEAKENED,
    INFECTED,
    FLAGGED,
}

fn parse(input: &Vec<&str>) -> (HashMap<(i64, i64), State>, (i64, i64)) {
    let mut infected: HashMap<(i64, i64), State> = HashMap::new();

    let mut row = 0;
    for line in input {
        let mut col = 0;
        for c in line.chars() {
            if c == '#' {
                infected.insert((row, col), State::INFECTED);
            }
            col += 1;
        }
        row += 1;
    }

    (infected, ((input.len() / 2) as i64, (input[0].chars().count() / 2) as i64))
}

fn day22_part1(input: &Vec<&str>, activity: u64) -> u64 {
    let (mut infection, mut position) = parse(input);
    let mut direction = (-1, 0);
    let mut infected = 0;

    for _ in 0..activity {
        match *infection.get(&position).unwrap_or(&State::CLEAN) {
            State::CLEAN => {
                direction = (direction.1 * -1, direction.0);
                infection.insert(position, State::INFECTED);
                infected += 1;
            }
            State::INFECTED => {
                direction = (direction.1, direction.0 * -1);
                infection.remove(&position);
            }
            _ => panic!()
        }

        position = (position.0 + direction.0, position.1 + direction.1);
    }

    infected
}

fn day22_part2(input: &Vec<&str>, activity: u64) -> u64 {
    let (mut infection, mut position) = parse(input);
    let mut direction = (-1, 0);
    let mut infected = 0;

    for _ in 0..activity {
        match *infection.get(&position).unwrap_or(&State::CLEAN) {
            State::CLEAN => {
                direction = (direction.1 * -1, direction.0);
                infection.insert(position, State::WEAKENED);
            }
            State::WEAKENED => {
                infection.insert(position, State::INFECTED);
                infected += 1;
            }
            State::INFECTED => {
                direction = (direction.1, direction.0 * -1);
                infection.insert(position, State::FLAGGED);
            }
            State::FLAGGED => {
                direction = (-direction.0, -direction.1);
                infection.insert(position, State::CLEAN);
            }
        }

        position = (position.0 + direction.0, position.1 + direction.1);
    }

    infected
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1: {}", day22_part1(&buffer.lines().collect(), 10000));
    println!("part2: {}", day22_part2(&buffer.lines().collect(), 10000000));
}