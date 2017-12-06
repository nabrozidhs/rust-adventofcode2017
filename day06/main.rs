use std::collections::HashMap;

fn max_position(input: &Vec<u32>) -> usize {
    let mut max = input[0];
    let mut index = 0;
    for i in 1..input.len() {
        if max < input[i] {
            max = input[i];
            index = i;
        }
    }
    return index;
}

fn day6(input: &Vec<u32>) -> (u32, u32) {
    let mut board = input.clone();
    let mut iterations = 0;
    let mut seen_boards = HashMap::new();

    while !seen_boards.contains_key(&board) {
        seen_boards.insert(board.clone(), iterations);

        iterations += 1;
        let mut position = max_position(&board);
        let mut value = board[position];
        board[position] = 0;
        while value > 0 {
            position += 1;
            let array_index = position % board.len();
            board[array_index] += 1;
            value -= 1;
        }
    }

    return (iterations, iterations - seen_boards.get(&board).unwrap());
}

fn main() {
    let input = vec!(4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3);

    let (part1, part2) = day6(&input);
    println!("part1: {} ", part1);
    println!("part2: {} ", part2);
}