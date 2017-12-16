use std::io;
use std::io::Read;

extern crate regex;

use regex::Regex;

struct State {
    array: Vec<char>,
    offset: i64,
}

impl State {
    fn new(array: Vec<char>) -> State {
        State { array, offset: 0 }
    }

    fn print(&self) -> String {
        let n: i64 = self.array.len() as i64;
        let mut new_array = Vec::with_capacity(self.array.len());
        for i in 0..self.array.len() {
            new_array.push(self.array[(((i as i64 - self.offset) % n + n) % n) as usize])
        }
        return new_array.into_iter().collect();
    }
}

enum Command {
    Spin(i64),
    Exchange(usize, usize),
    Partner(char, char),
}

fn run(command: &Command, state: &mut State) {
    match *command {
        Command::Spin(offset) => state.offset += offset,
        Command::Exchange(a, b) => {
            let n: i64 = state.array.len() as i64;
            let i: usize = (((a as i64 - state.offset) % n + n) % n) as usize;
            let k: usize = (((b as i64 - state.offset) % n + n) % n) as usize;
            let temp = state.array[i];
            state.array[i] = state.array[k];
            state.array[k] = temp;
        }
        Command::Partner(a, b) => {
            let i = state.array.iter().enumerate().find(|x| *x.1 == a).unwrap().0;
            let k = state.array.iter().enumerate().find(|x| *x.1 == b).unwrap().0;
            let temp = state.array[i];
            state.array[i] = state.array[k];
            state.array[k] = temp;
        }
    }
}

fn parse_commands(input: &Vec<&str>) -> Vec<Command> {
    let re_spin = Regex::new(r"^s(\d+)$").unwrap();
    let re_exchange = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
    let re_partner = Regex::new(r"^p(\w)/(\w)$").unwrap();

    let mut commands: Vec<Command> = vec!();
    for line in input {
        if re_spin.is_match(line) {
            commands.push(Command::Spin(re_spin.captures_iter(line).next().unwrap()[1].parse::<i64>().unwrap()));
        } else if re_exchange.is_match(line) {
            let cap = re_exchange.captures_iter(line).next().unwrap();
            commands.push(Command::Exchange(cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap()));
        } else if re_partner.is_match(line) {
            let cap = re_partner.captures_iter(line).next().unwrap();
            commands.push(Command::Partner(cap[1].parse::<char>().unwrap(), cap[2].parse::<char>().unwrap()));
        }
    }

    return commands;
}

fn process(commands: &Vec<Command>, state: &mut State) {
    for command in commands {
        run(command, state);
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<&str> = buffer.split(",").collect();

    let commands = parse_commands(&input);

    let mut state = State::new("abcdefghijklmnop".chars().collect());
    let mut moves: Vec<String> = vec!();
    moves.push(state.print());
    process(&commands, &mut state);
    println!("part1 {}", state.print());
    while !moves.contains(&state.print()) {
        moves.push(state.print());
        process(&commands, &mut state);
    }
    println!("part2 {}", moves[1000000000 % moves.len()]);
}