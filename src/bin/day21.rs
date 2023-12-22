use std::collections::HashSet;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day21/in_small.txt";
const IN: &str = "assets/day21/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 3600);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Start,
    Plot,
    Rock,
}

fn parse_input(file: &str) -> Res<(Point, Vec<Vec<Type>>)> {
    let lines = read_lines(file)?;

    let mut start = (-1, -1);

    let lines = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| match c {
                    '.' => Type::Plot,
                    'S' => {
                        start = (i as i32, j as i32);
                        Type::Start
                    }
                    '#' => Type::Rock,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((start, lines))
}

type Point = (i32, i32);

// UP, RIGHT, DOWN, LEFT
const DIRS: &[Point; 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

// 3600
fn part1(file: &str) -> Res<usize> {
    let (start, mtx) = parse_input(file)?;

    let mut hm = HashSet::<Point>::new();
    let mut hm_ = HashSet::<Point>::new();

    hm.insert(start);
    for _ in 1..=64 {
        for cur in hm.iter() {
            DIRS.iter()
                .filter_map(|dir| {
                    let to = (cur.0 + dir.0, cur.1 + dir.1);
                    let cell = mtx.get(to.0 as usize)?.get(to.1 as usize)?;
                    (cell != &Type::Rock).then_some(Some(to))
                })
                .for_each(|to| {
                    if let Some(to) = to {
                        hm_.insert(to);
                    }
                });
        }
        hm = hm_;
        hm_ = HashSet::new();
    }

    Ok(hm.len())
}

fn part2(file: &str) -> Res<u32> {
    Ok(42)
}
