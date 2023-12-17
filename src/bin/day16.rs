use std::collections::{HashMap, VecDeque};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day16/in_small.txt";
const IN: &str = "assets/day16/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 7074);
    assert!(ans_part_2 == 7530);

    Ok(())
}

type Direction = (i32, i32);
type Point = (i32, i32);

const UP: Direction = (-1, 0);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);

const DIRS: &[Direction; 4] = &[UP, RIGHT, DOWN, LEFT];

fn parse_input(file: &str) -> Res<Vec<Vec<u8>>> {
    let lines = read_lines(file)?;

    let lines = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect::<Vec<_>>();

    Ok(lines)
}

fn solve(lines: &Vec<Vec<u8>>, start: (Point, Direction)) -> usize {
    let n = lines.len() as i32;
    let m = lines[0].len() as i32;

    let mut q = VecDeque::<(Point, Direction)>::new();
    let mut energized = HashMap::<Point, [bool; 4]>::new();

    q.push_back(start);
    while let Some((pos, dir)) = q.pop_front() {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if next.0 < 0 || next.0 >= n || next.1 < 0 || next.1 >= m {
            continue;
        }

        let dir_index = DIRS.iter().position(|dir_| dir_ == &dir).unwrap();

        if let Some(already_energized) = energized.get_mut(&next) {
            if already_energized[dir_index] {
                continue;
            }
            already_energized[dir_index] = true;
        } else {
            let mut dirs = [false; 4];
            dirs[dir_index] = true;
            energized.insert(next, dirs);
        }

        match lines[next.0 as usize][next.1 as usize] {
            b'.' => {
                q.push_back((next, dir));
            }
            b'|' => match dir {
                LEFT | RIGHT => {
                    q.push_back((next, UP));
                    q.push_back((next, DOWN));
                }
                _ => q.push_back((next, dir)),
            },
            b'-' => match dir {
                UP | DOWN => {
                    q.push_back((next, LEFT));
                    q.push_back((next, RIGHT));
                }
                _ => q.push_back((next, dir)),
            },
            b'/' => match dir {
                UP => q.push_back((next, RIGHT)),
                RIGHT => q.push_back((next, UP)),
                DOWN => q.push_back((next, LEFT)),
                LEFT => q.push_back((next, DOWN)),
                _ => unreachable!(),
            },
            b'\\' => match dir {
                UP => q.push_back((next, LEFT)),
                RIGHT => q.push_back((next, DOWN)),
                DOWN => q.push_back((next, RIGHT)),
                LEFT => q.push_back((next, UP)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    energized.len()
}

// 7074
fn part1(file: &str) -> Res<usize> {
    let lines = parse_input(file)?;

    let ans = solve(&lines, ((0, -1), RIGHT));

    Ok(ans)
}

// 7530
fn part2(file: &str) -> Res<usize> {
    let lines = parse_input(file)?;

    let n = lines.len() as i32;
    let m = lines[0].len() as i32;

    let ans = (0..n)
        .flat_map(|i| vec![((i, -1), RIGHT), ((i, m), LEFT)].into_iter())
        .chain((0..m).flat_map(|j| vec![((-1, j), DOWN), ((n, j), UP)].into_iter()))
        .map(|start| solve(&lines, start))
        .max()
        .unwrap_or(0);

    Ok(ans)
}
