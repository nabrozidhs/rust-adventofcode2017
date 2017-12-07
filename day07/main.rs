use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct Node {
    id: String,
    parent: String,
    children: HashSet<String>,
    value: i32,
}

fn node_weight(tree: &HashMap<String, Node>, node_id: String) -> i32 {
    let node = tree.get(&node_id).unwrap();
    let mut weight = node.value;
    if node.children.is_empty() {
        return weight;
    }

    for child in node.children.iter() {
        weight += node_weight(tree, child.clone());
    }

    return weight;
}

fn group_by_weight<'a>(tree: &'a HashMap<String, Node>, node: &Node) -> HashMap<i32, Vec<&'a Node>> {
    let mut group_by_weight: HashMap<i32, Vec<&Node>> = HashMap::new();
    for child in node.children.iter() {
        let weight = node_weight(tree, child.clone());
        if !group_by_weight.contains_key(&weight) {
            group_by_weight.insert(weight, Vec::new());
        }
        let mut previous = group_by_weight.get(&weight).unwrap().clone();
        previous.insert(0, tree.get(child).unwrap());
        group_by_weight.insert(weight, previous);
    }
    group_by_weight
}

fn find_unbalanced_node<'a>(tree: &'a HashMap<String, Node>, root: &Node) -> &'a Node {
    let mut unbalanced_node: &Node = tree.get(&root.id).unwrap();
    loop {
        let grouped = group_by_weight(tree, &unbalanced_node);
        if grouped.len() == 1 {
            break
        }
        unbalanced_node = grouped.iter()
            .min_by(|i1, i2| i1.1.len().cmp(&i2.1.len()))
            .map(|k| k.1[0])
            .unwrap();
    }
    unbalanced_node
}

fn part2(tree: &HashMap<String, Node>, root: &Node) -> i32 {
    let grouped = group_by_weight(
        tree,
        tree.get(&find_unbalanced_node(tree, root).parent).unwrap());

    let balanced_weight = grouped.iter()
        .max_by(|i1, i2| i1.1.len().cmp(&i2.1.len()))
        .map(|k| k.0)
        .unwrap();

    let unbalanced_weight = grouped.iter()
        .min_by(|i1, i2| i1.1.len().cmp(&i2.1.len()))
        .map(|k| k.0)
        .unwrap();

    return grouped.get(unbalanced_weight).unwrap()[0].value + (balanced_weight - unbalanced_weight);
}

fn build_tree(input: &Vec<&str>) -> HashMap<String, Node> {
    let re = Regex::new(r"^(\w+) \((\d+)\)$").unwrap();
    let re_with_children = Regex::new(r"^(\w+) \((\d+)\) -> ([\w, ]+)$").unwrap();
    let mut tree: HashMap<String, Node> = HashMap::new();
    for line in input.iter() {
        if re.is_match(line) {
            let cap = re.captures_iter(line).next().unwrap();
            let id = cap[1].to_string();
            let value: i32 = cap[2].parse().unwrap();
            let new_node = match tree.get(&id) {
                Some(node) => Node { id: id.clone(), parent: node.parent.clone(), children: HashSet::new(), value },
                None => Node { id: id.clone(), parent: "".to_string(), children: HashSet::new(), value },
            };
            tree.insert(id, new_node);
        }
        if re_with_children.is_match(line) {
            let cap = re_with_children.captures_iter(line).next().unwrap();
            let id = cap[1].to_string();
            let value: i32 = cap[2].parse().unwrap();
            let mut children: HashSet<String> = HashSet::new();
            for item in cap[3].split(", ") {
                children.insert(item.to_string());
                let children_node = match tree.get(item) {
                    Some(c) => Node { id: c.id.clone(), parent: id.clone(), children: c.children.clone(), value: c.value },
                    None => Node { id: item.to_string(), parent: id.clone(), children: HashSet::new(), value: 0 }
                };
                tree.insert(item.to_string(), children_node);
            }

            let new_node = match tree.get(&id) {
                Some(node) => Node { id: id.clone(), parent: node.parent.clone(), children, value },
                None => Node { id: id.clone(), parent: "".to_string(), children, value },
            };
            tree.insert(id, new_node);
        }
    }
    tree
}

fn find_root<'a>(tree: &'a HashMap<String, Node>) -> &'a Node {
    let mut root = tree.values().into_iter().next().unwrap();
    while root.parent != "" {
        root = tree.get(&root.parent).unwrap();
    }
    root
}

fn day7(input: &Vec<&str>) -> (String, i32) {
    let tree = build_tree(input);
    let root = find_root(&tree);

    return (root.id.clone(), part2(&tree, &root));
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input: Vec<&str> = buffer.lines().collect();

    let (parent, weight) = day7(&input);
    println!("part1 {}", parent);
    println!("part2 {}", weight);
}