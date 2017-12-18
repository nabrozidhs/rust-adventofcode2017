use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;

extern crate crossbeam;

fn process(input: &Vec<&str>, process: u8, tx: &Sender<i64>, rx: &Receiver<i64>) -> i64 {
    let mut registers: HashMap<String, i64> = HashMap::new();
    registers.insert("p".to_string(), process as i64);

    let mut idx = 0;
    let mut sent_values = 0;

    loop {
        let cmd = input[idx].split_whitespace().collect::<Vec<&str>>();
        match cmd[0] {
            "set" => {
                let value: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), value);
                idx += 1;
            }
            "add" => {
                let a: i64 = *registers.get(cmd[1]).unwrap_or(&0);
                let b: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), a + b);
                idx += 1;
            }
            "mul" => {
                let a: i64 = *registers.get(cmd[1]).unwrap_or(&0);
                let b: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), a * b);
                idx += 1;
            }
            "mod" => {
                let a: i64 = *registers.get(cmd[1]).unwrap_or(&0);
                let b: i64 = cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0));
                registers.insert(cmd[1].to_string(), a % b);
                idx += 1;
            }
            "snd" => {
                tx.send(cmd[1].parse().unwrap_or(*registers.get(cmd[1]).unwrap_or(&0))).unwrap();
                sent_values += 1;
                idx += 1;
            }
            "rcv" => {
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(a) => registers.insert(cmd[1].to_string(), a),
                    Err(_) => {
                        println!("process {}, {}", process, sent_values);
                        return sent_values;
                    }
                };
                idx += 1;
            }
            "jgz" => {
                if cmd[1].parse().unwrap_or(*registers.get(cmd[1]).unwrap_or(&0)) > 0 {
                    idx = (idx as i64 + cmd[2].parse().unwrap_or(*registers.get(cmd[2]).unwrap_or(&0))) as usize;
                } else {
                    idx += 1;
                }
            }
            _ => panic!(),
        }
    }
}

fn day18_part1(input: &Vec<&str>) -> i64 {
    let (tx1, rx1) = channel::<i64>();
    let (_, rx2) = channel::<i64>();

    crossbeam::scope(|scope| {
        scope.spawn(move || process(&input, 0, &tx1, &rx2));
    });

    let mut last_received = 0;
    loop {
        match rx1.recv_timeout(Duration::from_secs(1)) {
            Ok(a) => last_received = a,
            Err(_) => return last_received,
        };
    }
}

fn day18_part2(input: &Vec<&str>) {
    let (tx1, rx1) = channel::<i64>();
    let (tx2, rx2) = channel::<i64>();

    crossbeam::scope(|scope| {
        scope.spawn(move || process(&input, 0, &tx1, &rx2));
        scope.spawn(move || process(&input, 1, &tx2, &rx1));
    });
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("part1: {}", day18_part1(&buffer.lines().collect()));
    day18_part2(&buffer.lines().collect());
}