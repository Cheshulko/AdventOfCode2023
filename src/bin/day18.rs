use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day18/in_small.txt";
const IN: &str = "assets/day18/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 66993);
    assert!(ans_part_2 == 177243763226648);

    Ok(())
}

type Direction = (i128, i128);

const UP: Direction = (-1, 0);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);

const DIRS: &[Direction; 4] = &[RIGHT, DOWN, LEFT, UP];

fn parse_input1(file: &str) -> Res<Vec<(Direction, i128)>> {
    let lines = read_lines(file)?;

    let lines = lines
        .into_iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();

            let dir = match line_iter.next().unwrap().chars().next().unwrap() {
                'U' => UP,
                'D' => DOWN,
                'R' => RIGHT,
                'L' => LEFT,
                _ => unreachable!(),
            };
            let value = line_iter.next().unwrap().parse::<i128>().unwrap();

            (dir, value)
        })
        .collect::<Vec<_>>();

    Ok(lines)
}

fn parse_input2(file: &str) -> Res<Vec<(Direction, i128)>> {
    let lines = read_lines(file)?;

    let lines = lines
        .into_iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();

            line_iter.next();
            line_iter.next();

            let color = line_iter.next().unwrap();
            let color = &color[2..(color.len() - 1)];

            let value = i128::from_str_radix(&color[..5], 16).unwrap();
            let dir = usize::from_str_radix(&color[5..], 16).unwrap();

            (DIRS[dir], value)
        })
        .collect::<Vec<_>>();

    Ok(lines)
}

fn solve(data: Vec<((i128, i128), i128)>) -> i128 {
    let ((_, _), p, ans) = data.into_iter().fold(
        ((0, 0), 0, 0),
        |((cur_i, cur_j), mut p, mut ans), ((di, dj), steps)| {
            p += steps;
            let (next_i, next_j) = (cur_i + steps * di, cur_j + steps * dj);

            ans += (next_i - cur_i) * (next_j + cur_j);
            ((next_i, next_j), p, ans)
        },
    );

    ans.abs() / 2 + p / 2 + 1
}

// 66993
fn part1(file: &str) -> Res<i128> {
    Ok(solve(parse_input1(file)?))
}

fn part2(file: &str) -> Res<i128> {
    Ok(solve(parse_input2(file)?))
}
