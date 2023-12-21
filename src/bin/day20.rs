use std::collections::VecDeque;

use std::hash::{Hash, Hasher};
use std::usize;
use std::{collections::HashMap, vec};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day20/in_small.txt";
const IN: &str = "assets/day20/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 730797576);
    assert!(ans_part_2 == 226732077152351);

    Ok(())
}

fn parse_input(file: &str) -> Res<(Vec<Node>, HashMap<String, usize>, Vec<Vec<usize>>)> {
    fn parse_node(str: &str) -> Node {
        let t = str.chars().next().unwrap();

        let (t, name) = if t == '%' || t == '&' {
            let name = str[1..].to_owned();

            let t = match t {
                '%' => Type::FlipFlop(false),
                '&' => Type::Conjunction(vec![]),
                _ => unreachable!(),
            };

            (t, name)
        } else {
            if str == "broadcaster" {
                (Type::Start, str.to_owned())
            } else {
                (Type::None, str.to_owned())
            }
        };

        Node { t, name }
    }

    let lines = read_lines(file)?;

    let mut index_to_node: Vec<Node> = vec![];
    let mut node_to_index: HashMap<String, usize> = HashMap::<String, usize>::new();

    fn set_node(
        node: &Node,
        node_to_index: &mut HashMap<String, usize>,
        index_to_node: &mut Vec<Node>,
    ) -> bool {
        if !node_to_index.contains_key(&node.name) {
            node_to_index.insert(node.name.clone(), index_to_node.len());
            index_to_node.push(node.clone());
            true
        } else {
            false
        }
    }

    for line in lines.iter() {
        let mut line_iter = line.split(" -> ");
        let from = line_iter.next().unwrap();

        let node = parse_node(from);
        if !set_node(&node, &mut node_to_index, &mut index_to_node) {
            let index = *node_to_index.get(&node.name).unwrap();
            let node_old = &mut index_to_node[index];
            node_old.t = node.t;
        }

        for to in line_iter.next().unwrap().split(", ") {
            let node = parse_node(to);
            let _ = set_node(&node, &mut node_to_index, &mut index_to_node);
        }
    }

    let mut edges: Vec<Vec<usize>> = vec![vec![]; index_to_node.len()];

    for line in lines.iter() {
        let mut line_iter = line.split(" -> ");

        let from = line_iter.next().unwrap();
        let tos = line_iter.next().unwrap();

        let from = parse_node(from);
        let from_index = *node_to_index.get(&from.name).unwrap();

        for to in tos.split(", ").map(|to| {
            let to = parse_node(to);
            let to_index = *node_to_index.get(&to.name).unwrap();

            let node = &mut index_to_node[to_index];
            match node.t {
                Type::Conjunction(ref mut v) => v.push((from_index, false)),
                _ => {}
            }

            to_index
        }) {
            edges[from_index].push(to);
        }
    }

    Ok((index_to_node, node_to_index, edges))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Type {
    None,
    Start,
    FlipFlop(bool),
    Conjunction(Vec<(usize, bool)>),
}

#[derive(Debug, Clone)]
struct Node {
    pub t: Type,
    pub name: String,
}

impl Node {
    fn process(&mut self, signal: bool, from_ind: usize) -> Option<bool> {
        match self.t {
            Type::Start => Some(signal),
            Type::FlipFlop(ref mut state) => {
                if !signal {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            Type::Conjunction(ref mut states) => {
                if let Some(from) = states.iter_mut().find(|x| x.0 == from_ind) {
                    *from = (from.0, signal);
                }
                Some(!states.iter().all(|x| x.1))
            }
            Type::None => None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for Node {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    pub from: usize,
    pub tos: Vec<usize>,
}

// 730797576
fn part1(file: &str) -> Res<u64> {
    let (mut index_to_node, node_to_index, edges) = parse_input(file)?;

    let start_ind = *node_to_index.get("broadcaster").unwrap();

    let mut pulses = [0, 0];
    let mut q = VecDeque::<(usize, usize, bool)>::new();

    for _ in 0..1000 {
        q.push_back((0, start_ind, false));

        while let Some((from_ind, cur_ind, cur_signal)) = q.pop_front() {
            pulses[cur_signal as usize] += 1;
            let node = &mut index_to_node[cur_ind];

            if let Some(next_signal) = node.process(cur_signal, from_ind) {
                for to in &edges[cur_ind] {
                    q.push_back((cur_ind, *to, next_signal));
                }
            }
        }
    }

    Ok(pulses[0] * pulses[1])
}

// 226732077152351
fn part2(file: &str) -> Res<u64> {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while a != 0 {
            if a < b {
                std::mem::swap(&mut a, &mut b);
            }
            a %= b;
        }
        b
    }

    let (mut index_to_node, node_to_index, edges) = parse_input(file)?;

    // Bullshit
    let targets = ["rk", "cd", "qx", "zf"]
        .iter()
        .map(|x| *node_to_index.get(*x).unwrap())
        .collect::<Vec<_>>();
    let mut cycles = vec![0; 4];
    let mut found = 0;

    let start_ind = *node_to_index.get("broadcaster").unwrap();

    let mut q = VecDeque::<(usize, usize, bool)>::new();

    let mut pressed = 0;
    while found != 4 {
        pressed += 1;
        q.push_back((0, start_ind, false));

        while let Some((from_ind, cur_ind, cur_signal)) = q.pop_front() {
            let node = &mut index_to_node[cur_ind];

            if node.name == "rx" && cur_signal == false {
                return Ok(pressed);
            }

            if let Some(next_signal) = node.process(cur_signal, from_ind) {
                if let Some(ind) = targets.iter().position(|x| x == &cur_ind) {
                    if next_signal && cycles[ind] == 0 {
                        cycles[ind] = pressed;
                        found += 1;
                    }
                }
                for to in &edges[cur_ind] {
                    q.push_back((cur_ind, *to, next_signal));
                }
            }
        }
    }

    Ok(cycles.into_iter().fold(1, |ans, x| ans * x / gcd(ans, x)))
}
