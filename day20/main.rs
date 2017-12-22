extern crate regex;

use std::io;
use std::io::Read;

use regex::Regex;

struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl Particle {
    fn new(p: (i64, i64, i64), v: (i64, i64, i64), a: (i64, i64, i64)) -> Particle {
        Particle { p, v, a }
    }

    fn update(&mut self) {
        self.v = (self.v.0 + self.a.0, self.v.1 + self.a.1, self.v.2 + self.a.2);
        self.p = (self.v.0 + self.p.0, self.v.1 + self.p.1, self.v.2 + self.p.2);
    }

    fn distance_to(&self, other: (i64, i64, i64)) -> i64 {
        (self.p.0 - other.0).abs() + (self.p.1 - other.1).abs() + (self.p.2 - other.2).abs()
    }
}

fn parse(input: &Vec<&str>) -> Vec<Particle> {
    let mut particles: Vec<Particle> = vec!();

    let re = Regex::new(r"(\-?\d+,\-?\d+,\-?\d+)").unwrap();
    for line in input {
        let mut captures = re.captures_iter(line);
        let p_capture = captures.next().unwrap()[0].parse::<String>().unwrap();
        let v_capture = captures.next().unwrap()[0].parse::<String>().unwrap();
        let a_capture = captures.next().unwrap()[0].parse::<String>().unwrap();

        let p_split = p_capture.split(",").collect::<Vec<&str>>();
        let v_split = v_capture.split(",").collect::<Vec<&str>>();
        let a_split = a_capture.split(",").collect::<Vec<&str>>();

        particles.push(Particle::new(
            (p_split[0].parse().unwrap(), p_split[1].parse().unwrap(), p_split[2].parse().unwrap()),
            (v_split[0].parse().unwrap(), v_split[1].parse().unwrap(), v_split[2].parse().unwrap()),
            (a_split[0].parse().unwrap(), a_split[1].parse().unwrap(), a_split[2].parse().unwrap())));
    }

    particles
}

fn day20_part1(input: &Vec<&str>, num_iterations: u64) -> usize {
    let mut particles = parse(input);

    for _ in 0..num_iterations {
        for i in 0..particles.len() {
            particles[i].update();
        }
    }

    let mut closest = particles[0].distance_to((0, 0, 0));
    let mut index = 0;
    for i in 1..particles.len() {
        let next = particles[i].distance_to((0, 0, 0));
        if next < closest {
            index = i;
            closest = next;
        }
    }

    index
}

fn day20_part2(input: &Vec<&str>, num_iterations: u64) -> usize {
    let mut particles = parse(input);

    for _ in 0..num_iterations {
        for i in 0..particles.len() {
            particles[i].update();
        }

        // check collisions
        let mut i: i64 = particles.len() as i64 - 1;
        while i > 0 {
            let mut found = false;
            let p = particles[i as usize].p;
            let mut k: i64 = i - 1;
            while k >= 0 {
                if p == particles[k as usize].p {
                    particles.remove(k as usize);
                    i -= 1;
                    found = true;
                }
                k -= 1;
            }
            if found {
                particles.remove(i as usize);
            }
            i -= 1;
        }
    }

    particles.len()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1: {}", day20_part1(&buffer.lines().collect(), 10000));
    println!("part2: {}", day20_part2(&buffer.lines().collect(), 1000));
}