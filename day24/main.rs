use std::io;
use std::io::Read;

type Component = (u64, u64);

fn parse(input: &Vec<&str>) -> Vec<Component> {
    let mut output: Vec<Component> = vec!();

    for line in input {
        let splitted = line.split("/").collect::<Vec<&str>>();

        output.push((splitted[0].parse().unwrap(), splitted[1].parse().unwrap()));
    }

    output
}

fn build_bridges(current: &Component, components: &Vec<Component>) -> Vec<Vec<Component>> {
    let mut bridges: Vec<Vec<Component>> = vec!();
    for i in 0..components.len() {
        let mut component = components[i];
        let mut found = false;
        if current.1 == component.0 {
            found = true;
        } else if current.1 == component.1 {
            found = true;
            component = (component.1, component.0);
        }

        if found {
            let mut copy = components.clone();
            copy.remove(i);
            bridges.push(vec!(component));
            for mut b in build_bridges(&component, &copy) {
                b.insert(0, component);
                bridges.push(b);
            }
        }
    }
    bridges
}

fn bridge_strength(bridge: &Vec<Component>) -> u64 {
    bridge.iter().map(|c| c.0 + c.1).sum()
}

fn day24(input: &Vec<&str>) -> (u64, u64) {
    let mut bridges = build_bridges(&(0, 0), &parse(&input));

    bridges.sort_by_key(|b| bridge_strength(b));

    let part1 = bridge_strength(&bridges[bridges.len() - 1]);

    bridges.sort_by_key(|b| b.len());
    (part1, bridge_strength(&bridges[bridges.len() - 1]))
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let (part1, part2) = day24(&buffer.lines().collect());
    println!("part1 {}", part1);
    println!("part2 {}", part2);
}