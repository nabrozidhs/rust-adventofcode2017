use std::io;
use std::io::Read;

fn day19(input: &Vec<Vec<char>>) -> (String, i32) {
    let mut position = (0, 0);
    let mut direction: (i32, i32) = (0, 1);
    for i in 0..input[0].len() {
        if input[0][i] == '|' {
            position = (i, 0);
            break;
        }
    }

    let mut output = "".to_string();
    let mut steps = 0;
    while input[position.1][position.0] != ' ' {
        steps += 1;
        position = ((position.0 as i32 + direction.0) as usize, (position.1 as i32 + direction.1) as usize);

        let next_value = input[position.1][position.0];
        match next_value {
            '|' | '-' => {}
            '+' => {
                match direction {
                    (1, 0) | (-1, 0) => {
                        if position.1 >= 1 && position.0 < input[position.1 - 1].len() && input[position.1 - 1][position.0] != ' ' {
                            direction = (0, -1);
                        } else {
                            direction = (0, 1);
                        }
                    }
                    _ => {
                        if position.0 >= 1 && input[position.1][position.0 - 1] != ' ' {
                            direction = (-1, 0);
                        } else {
                            direction = (1, 0);
                        }
                    }
                }
            }
            _ => output = format!("{}{}", output, next_value)
        }
    }

    (output, steps)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<Vec<char>> = buffer.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let (part1, part2) = day19(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}