use std::collections::HashMap;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day8/in_small.txt";
const IN: &str = "assets/day8/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

type Rules = HashMap<String, [String; 2]>;

fn parse_input(file: &str) -> Res<(Vec<usize>, Rules)> {
    let lines = read_lines(file)?;
    let lines = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut lines_iter = lines.into_iter();

    let instructions = lines_iter
        .next()
        .expect("Could not parse instructions")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let rules = lines_iter
        .map(|line| {
            let rule = line.split('=');
            let mut rule_iter = rule.into_iter();
            let from = rule_iter
                .next()
                .expect("Could not parse `from`")
                .trim()
                .to_string();
            let to = rule_iter
                .next()
                .expect("Could not parse `to`")
                .trim_matches([' ', '(', ')'].as_slice());

            let mut to = to.split(", ");

            (
                from,
                [
                    to.next().expect("Could not parse to-left").to_string(),
                    to.next().expect("Could not parse to-right").to_string(),
                ],
            )
        })
        .fold(HashMap::new(), |mut hm, (from, to)| {
            hm.insert(from, to);
            hm
        });

    Ok((instructions, rules))
}

// 13207
fn part1(file: &str) -> Res<u64> {
    let (instructions, rules) = parse_input(file)?;

    let instructions_len = instructions.len();
    let mut instruction_ind = 0;
    let mut cnt = 0;
    let mut cur = "AAA";

    while cur != "ZZZ" {
        let next = rules.get(cur).unwrap();
        cur = &next[instructions[instruction_ind]];
        instruction_ind = (instruction_ind + 1) % instructions_len;
        cnt += 1;
    }

    Ok(cnt)
}

// 12324145107121
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

    let (instructions, rules) = parse_input(file)?;

    let instructions_len = instructions.len();
    let starts = rules
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    let cnt = starts
        .into_iter()
        .map(|mut cur| {
            let mut instruction_ind = 0;
            let mut cnt = 0;

            while !cur.ends_with("Z") {
                let next = rules.get(cur).unwrap();
                cur = &next[instructions[instruction_ind]];
                instruction_ind = (instruction_ind + 1) % instructions_len;
                cnt += 1;
            }

            cnt
        })
        .fold(1, |ans, x| ans * x / gcd(ans, x));

    Ok(cnt)
}
