use std::collections::HashMap;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day7/in_small.txt";
const IN: &str = "assets/day7/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 248453531);
    assert!(ans_part_2 == 248781813);

    Ok(())
}

type Cards = HashMap<char, u32>;

fn parse_input(lines: &Vec<String>) -> Res<Vec<(Cards, String, u32)>> {
    let result = lines
        .iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace().into_iter();
            let hand = line_iter.next().unwrap();
            let bid = line_iter.next().unwrap();

            let hand_cards = hand.chars().fold(HashMap::new(), |mut hm, card| {
                *hm.entry(card).or_insert(0) += 1;
                hm
            });
            let bid = bid.parse::<u32>().ok().expect("Could not parse a bid");

            (hand_cards, hand.to_string(), bid)
        })
        .collect::<Vec<_>>();

    Ok(result)
}

const KINDS: &[char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '.',
];

const KINDS_NO_J: &[char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn has_five_of_a_kind(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    KINDS_NO_J
        .iter()
        .any(|kind| cards.get(kind).unwrap_or(&0) + j_cnt == 5)
}

fn has_four_of_a_kind(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    KINDS_NO_J
        .iter()
        .any(|kind| cards.get(kind).unwrap_or(&0) + j_cnt == 4)
}

fn has_full_house(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    let three = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &3)
        .count();

    let two = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &2)
        .count();

    (three == 1 && two == 1)
        || (two == 2 && j_cnt == &1)
        || (three == 1 && j_cnt == &1)
        || (two == 1 && j_cnt == &2)
        || j_cnt == &3
}

fn has_three_of_a_kind(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    let three = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &3)
        .count();
    let two = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &2)
        .count();

    three == 1 || (two == 1 && j_cnt == &1) || j_cnt == &2
}

fn has_two_pair(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    let two = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &2)
        .count();
    two == 2 || (two == 1 && j_cnt == &1)
}

fn has_one_pair(cards: &Cards) -> bool {
    let j_cnt = cards.get(&'.').unwrap_or(&0);

    let two = KINDS_NO_J
        .iter()
        .filter(|kind| cards.get(kind).unwrap_or(&0) == &2)
        .count();
    two == 1 || j_cnt == &1
}

fn solve(mut data: Vec<(Cards, String, u32)>) -> Res<u32> {
    let arr: Vec<&dyn Fn(&Cards) -> bool> = vec![
        &has_five_of_a_kind,
        &has_four_of_a_kind,
        &has_full_house,
        &has_three_of_a_kind,
        &has_two_pair,
        &has_one_pair,
    ];

    data.sort_by(|(cards_a, hand_a, _), (cards_b, hand_b, _)| {
        let a_ = arr.iter().position(|f| (f)(&cards_a)).unwrap_or(10);
        let b_ = arr.iter().position(|f| (f)(&cards_b)).unwrap_or(10);

        match b_.cmp(&a_) {
            std::cmp::Ordering::Equal => hand_a
                .chars()
                .zip(hand_b.chars())
                .find(|(a, b)| a != b)
                .map(|(a, b)| {
                    let kind_a = KINDS.iter().position(|x| x == &a).unwrap();
                    let kind_b = KINDS.iter().position(|x| x == &b).unwrap();

                    kind_b.cmp(&kind_a)
                })
                .unwrap(),
            n @ _ => n,
        }
    });

    Ok(data
        .into_iter()
        .enumerate()
        .map(|(ind, (_, _, bid))| (1 + ind as u32) * bid)
        .sum::<u32>())
}

// 248453531
fn part1(file: &str) -> Res<u32> {
    let lines = read_lines(file)?;

    Ok(solve(parse_input(&lines)?)?)
}

// 248781813
fn part2(file: &str) -> Res<u32> {
    let lines = read_lines(file)?;
    let lines = lines
        .into_iter()
        .map(|line| line.replace("J", "."))
        .collect();

    Ok(solve(parse_input(&lines)?)?)
}
