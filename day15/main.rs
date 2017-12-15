struct Gen {
    next: u64,
    factor: u64,
}

impl Gen {
    fn new(next: u64, factor: u64) -> Gen {
        Gen { next, factor }
    }

    fn next(&mut self) -> u32 {
        let next = (self.next * self.factor) % 2147483647;
        self.next = next;
        next as u32
    }
}

fn day15(gen_a: &mut Gen, gen_b: &mut Gen, op: usize, next_f: fn(&mut Gen, &mut Gen) -> (u32, u32)) -> u32 {
    let mut sum = 0;
    for _ in 0..op {
        let (a, b) = next_f(gen_a, gen_b);
        if a % 65536 == b % 65536 {
            sum += 1
        }
    }
    sum
}

fn part1(gen_a: &mut Gen, gen_b: &mut Gen) -> (u32, u32) {
    (gen_a.next(), gen_b.next())
}

fn part2(gen_a: &mut Gen, gen_b: &mut Gen) -> (u32, u32) {
    let mut a = gen_a.next();
    while a % 4 != 0 {
        a = gen_a.next();
    }

    let mut b = gen_b.next();
    while b % 8 != 0 {
        b = gen_b.next();
    }

    (a, b)
}

fn main() {
    println!("part1 {}", day15(&mut Gen::new(116, 16807), &mut Gen::new(299, 48271), 40_000_000, part1));
    println!("part2 {}", day15(&mut Gen::new(116, 16807), &mut Gen::new(299, 48271), 5_000_000, part2));
}