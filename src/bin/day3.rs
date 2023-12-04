use std::collections::{HashMap, HashSet};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day3/in_small.txt";
const IN: &str = "assets/day3/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

type Position = (i32, i32);

fn extract_symbols(file: &str) -> Res<(Vec<String>, HashSet<Position>)> {
    let mut symbols: HashSet<Position> = HashSet::new();
    let mut lines = read_lines(file)?;

    for (line_ind, line) in lines.iter_mut().enumerate() {
        *line = line
            .char_indices()
            .map(|(ind, c)| {
                if !c.is_ascii_digit() && c != '.' {
                    symbols.insert((line_ind as i32, ind as i32));
                    '.'
                } else {
                    c
                }
            })
            .collect();
    }

    Ok((lines, symbols))
}

const DIRS: std::ops::Range<i32> = -1..2;

// 540131
fn part1(file: &str) -> Res<u32> {
    let (lines, symbols) = extract_symbols(file)?;

    let mut sum = 0;
    for (row_i, line) in lines.iter().enumerate() {
        let mut pos = 0;
        for str in line.split('.').collect::<Vec<_>>() {
            if str.is_empty() {
                pos += 1;
                continue;
            }

            let num_len = str.len();
            if let Ok(number) = str.parse::<u32>() {
                'out: for row_j in pos..(pos + num_len) {
                    for di in DIRS {
                        for dj in DIRS {
                            let to_i = row_i as i32 + di;
                            let to_j = row_j as i32 + dj;
                            if symbols.contains(&(to_i, to_j)) {
                                sum += number;
                                break 'out;
                            }
                        }
                    }
                }
            }

            pos += num_len;
            pos += 1;
        }
    }

    Ok(sum)
}

// 86879020
fn part2(file: &str) -> Res<u32> {
    let (lines, symbols) = extract_symbols(file)?;
    let mut symbol_numbers: HashMap<Position, HashSet<u32>> = HashMap::new();

    for (row_i, line) in lines.iter().enumerate() {
        let x = line.split('.').collect::<Vec<_>>();

        let mut pos = 0;
        for str in x {
            if str.is_empty() {
                pos += 1;
                continue;
            }

            let num_len = str.len();
            if let Ok(number) = str.parse::<u32>() {
                for row_j in pos..(pos + num_len) {
                    for di in DIRS {
                        for dj in DIRS {
                            let to_i = row_i as i32 + di;
                            let to_j = row_j as i32 + dj;
                            if symbols.contains(&(to_i, to_j)) {
                                symbol_numbers
                                    .entry((to_i, to_j))
                                    .or_insert(HashSet::new())
                                    .insert(number);
                            }
                        }
                    }
                }
            }

            pos += num_len;
            pos += 1;
        }
    }

    Ok(symbol_numbers
        .into_iter()
        .filter(|(_, value)| value.len() == 2)
        .map(|(_, value)| value.into_iter().product::<u32>())
        .sum())
}
