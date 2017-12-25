use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    state: char,
    remaining: u64,
    current_position: i64,
    enabled_matrix: HashSet<i64>,
}

fn parse_state_actions(input: &Vec<&str>) -> Box<Fn(&mut State)> {
    let f_disabled = parse_actions(&input[2..5].to_vec());
    let f_enabled = parse_actions(&input[6..9].to_vec());
    Box::new(move |state: &mut State| {
        if state.enabled_matrix.contains(&state.current_position) {
            f_enabled(state);
        } else {
            f_disabled(state);
        }
        state.remaining -= 1;
    })
}

fn parse_actions(input: &Vec<&str>) -> Box<Fn(&mut State)> {
    let value = input[0].contains('1');
    let to_right = input[1].contains("right");
    let to_state: char = input[2].chars().nth(input[2].len() - 2).unwrap();

    Box::new(move |state: &mut State| {
        if value {
            state.enabled_matrix.insert(state.current_position);
        } else {
            state.enabled_matrix.remove(&state.current_position);
        }

        state.current_position = state.current_position + if to_right { 1 } else { -1 };

        state.state = to_state;
    })
}

fn parse(input: &Vec<&str>) -> (State, HashMap<char, Box<Fn(&mut State)>>) {
    let header = input[0].lines().collect::<Vec<&str>>();
    let state = header[0].chars().collect::<Vec<char>>()[header[0].len() - 2];
    let steps = header[1].split_whitespace().collect::<Vec<&str>>();
    let remaining = steps[steps.len() - 2].parse().unwrap();

    let mut operations: HashMap<char, Box<Fn(&mut State)>> = HashMap::new();
    for i in 1..input.len() {
        let lines: Vec<&str> = input[i].lines().collect();
        operations.insert(lines[0].chars().nth(lines[0].len() - 2).unwrap(),
                          parse_state_actions(&lines));
    }

    (State { state, remaining, current_position: 0, enabled_matrix: HashSet::new() }, operations)
}

fn day25(input: &Vec<&str>) -> usize {
    let (mut state, operations) = parse(input);

    while state.remaining > 0 {
        let f = operations.get(&state.state).unwrap();
        f(&mut state);
    }

    state.enabled_matrix.len()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1 {}", day25(&buffer.split("\n\n").collect()));
}