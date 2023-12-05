use std::collections::HashMap;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day4/in_small.txt";
const IN: &str = "assets/day4/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

fn parse(file: &str) -> Res<Vec<(HashMap<u32, u32>, HashMap<u32, u32>)>> {
    fn parse_numbers(str: &str) -> HashMap<u32, u32> {
        str.split_whitespace()
            .filter_map(|number_str| number_str.parse::<u32>().ok())
            .fold(<HashMap<u32, u32>>::new(), |mut hm, number| {
                *hm.entry(number).or_insert(0) += 1;
                hm
            })
    }

    let mut result = vec![];

    for line in read_lines(file)?.into_iter() {
        let mut line_iter = line.split([':', '|']);
        let _ = line_iter.next();

        let winning_str = line_iter.next().unwrap_or_default();
        let winning = parse_numbers(winning_str);

        let ours_str = line_iter.next().unwrap_or_default();
        let ours = parse_numbers(ours_str);

        result.push((winning, ours));
    }

    Ok(result)
}

// 25004
fn part1(file: &str) -> Res<u32> {
    Ok(parse(file)?
        .into_iter()
        .map(|(winning, ours)| {
            ours.into_iter()
                .filter(|(key, _)| winning.contains_key(key))
                .fold(
                    0,
                    |result, (_, count)| {
                        if result > 0 {
                            result << count
                        } else {
                            1
                        }
                    },
                )
        })
        .sum())
}

// 14427616
fn part2(file: &str) -> Res<u32> {
    let cards = parse(file)?;
    let mut cards_type_cnt = vec![1; cards.len()];

    Ok(cards
        .into_iter()
        .enumerate()
        .map(|(ind, (winning, ours))| {
            let wins = ours
                .into_iter()
                .filter_map(|(number, count)| {
                    if winning.contains_key(&number) {
                        Some(count)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for next_ind in (ind + 1)..((ind + 1 + wins as usize).min(cards_type_cnt.len())) {
                cards_type_cnt[next_ind] += cards_type_cnt[ind];
            }

            cards_type_cnt[ind]
        })
        .sum())
}
