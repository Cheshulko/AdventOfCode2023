use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day2/in_small.txt";
const IN: &str = "assets/day2/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

const CONS: &[(&str, u32)] = &[("red", 12), ("green", 13), ("blue", 14)];

type TurnType = (u32, String); // count, color
type Game = (u32, Vec<Vec<TurnType>>); // index, turns

fn parse_game(file: &str) -> Res<Vec<Game>> {
    let lines = read_lines(file)?
        .into_iter()
        .map(|line| {
            let games = line.split([':', ';']);
            let mut game_iter = games.into_iter().map(|game| game.trim().to_string());
            let game_ind = game_iter.next().expect("Could not parse game index");
            let game_ind = game_ind
                .split(' ')
                .next_back()
                .expect("Could not parse game index")
                .parse::<u32>()
                .expect("Could not parse game index");

            (
                game_ind,
                game_iter
                    .map(|game| {
                        game.split(',')
                            .map(|game_| {
                                let game_ = game_.trim();
                                let mut game_iter = game_.split(' ');
                                let cnt = game_iter
                                    .next()
                                    .expect("Could not get count")
                                    .parse::<u32>()
                                    .expect("Could not parse count");
                                let color =
                                    game_iter.next().expect("Could not get color").to_string();
                                (cnt, color)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    Ok(lines)
}

// 3099
fn part1(file: &str) -> Res<u32> {
    Ok(parse_game(file)?
        .into_iter()
        .filter_map(|(ind, game)| {
            let games_flatten = game
                .iter()
                .flat_map(|game_turn| game_turn.iter())
                .collect::<Vec<_>>();

            if CONS.iter().all(|(color, mx)| {
                games_flatten
                    .iter()
                    .filter_map(|game_| {
                        if game_.1.as_str() == *color {
                            Some(game_.0)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(u32::MAX)
                    <= *mx
            }) {
                Some(ind)
            } else {
                None
            }
        })
        .sum())
}

// 72970
fn part2(file: &str) -> Res<u32> {
    Ok(parse_game(file)?
        .into_iter()
        .map(|(_, game)| {
            let game_flat = game
                .iter()
                .flat_map(|game_turn| game_turn.iter())
                .collect::<Vec<_>>();

            CONS.iter()
                .map(|(color, _)| {
                    game_flat
                        .iter()
                        .filter_map(|game_| {
                            if game_.1.as_str() == *color {
                                Some(game_.0)
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(0)
                })
                .product::<u32>()
        })
        .sum())
}
