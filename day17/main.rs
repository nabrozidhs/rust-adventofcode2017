fn day17_part1(steps: usize, num_inserts: u32) -> u32 {
    let mut array = vec!(0);
    let mut i: usize = 0;

    for n in 1..(num_inserts + 1) {
        i = (i + steps) % array.len();
        array.insert(i + 1, n);
        i += 1;
    }

    for i in 0..array.len() {
        if array[i] == num_inserts {
            return array[(i + 1) % array.len()];
        }
    }
    panic!();
}

fn day17_part2(steps: usize, num_inserts: usize) -> usize {
    let mut last_zero_index = 0;
    let mut index = 0;
    for i in 0..num_inserts {
        index = (index + steps) % (i + 1);
        if index == 0 {
            last_zero_index = i + 1;
        }
        index += 1;
    }
    return last_zero_index;
}

fn main() {
    println!("part1 {}", day17_part1(386, 2017));
    println!("part2 {}", day17_part2(386, 50000000));
}