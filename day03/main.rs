use std::collections::HashMap;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Part {
    Part1,
    Part2
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn surrounding_sum(board: &HashMap<Point, u64>, position: Point, part: Part) -> u64 {
    // ignore for first part as it overflows.
    if part == Part::Part1 {
        return 0;
    }

    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            sum += *board.get(&Point { x: position.x + i, y: position.y + j }).unwrap_or(&0);
        }
    }
    return sum;
}

fn day3(input: i32, part: Part) -> u64 {
    let starting_position = Point { x: 0, y: 0 };

    let mut board: HashMap<Point, u64> = HashMap::new();
    board.insert(starting_position, 1);

    let mut i = 1;
    let mut direction = Direction::Right;
    let mut position = starting_position;
    let mut sum: u64 = 0;
    while (part == Part::Part1 && i < input) || (part == Part::Part2 && sum < input as u64) {
        i += 1;
        match direction {
            Direction::Right => {
                position = Point { x: position.x + 1, y: position.y };
                sum = surrounding_sum(&board, position, part);
                board.insert(position, sum);

                // try head up
                if !board.contains_key(&Point { x: position.x, y: position.y + 1 }) {
                    direction = Direction::Up;
                }
            }
            Direction::Up => {
                position = Point { x: position.x, y: position.y + 1 };
                sum = surrounding_sum(&board, position, part);
                board.insert(position, sum);

                // try head left
                if !board.contains_key(&Point { x: position.x - 1, y: position.y }) {
                    direction = Direction::Left;
                }
            }
            Direction::Left => {
                position = Point { x: position.x - 1, y: position.y };
                sum = surrounding_sum(&board, position, part);
                board.insert(position, sum);

                // try head down
                if !board.contains_key(&Point { x: position.x, y: position.y - 1 }) {
                    direction = Direction::Down;
                }
            }
            Direction::Down => {
                position = Point { x: position.x, y: position.y - 1 };
                sum = surrounding_sum(&board, position, part);
                board.insert(position, sum);

                // try head right
                if !board.contains_key(&Point { x: position.x + 1, y: position.y }) {
                    direction = Direction::Right;
                }
            }
        }
    }

    return match part {
        Part::Part1 => {
            ((starting_position.x - position.x).abs() + (starting_position.y - position.y).abs()) as u64
        }
        Part::Part2 => { sum }
    };
}

fn main() {
    let input = 289326;

    println!("part1: {}", day3(input, Part::Part1));
    println!("part2: {}", day3(input, Part::Part2));
}