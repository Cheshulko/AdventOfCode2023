use std::{cmp::Reverse, collections::BinaryHeap};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day17/in_small.txt";
const IN: &str = "assets/day17/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 1039);
    assert!(ans_part_2 == 1201);

    Ok(())
}

type Direction = (i32, i32);

const UP: Direction = (-1, 0);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);

const DIRS: &[Direction; 4] = &[UP, RIGHT, DOWN, LEFT];

fn parse_input(file: &str) -> Res<Vec<Vec<i32>>> {
    let lines = read_lines(file)?;

    let lines = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| ((c as u8) - b'0') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(lines)
}

type Position = (i32, i32);
type DirectionIndex = usize;
// [position i][position j][streak][direction index FROM WHERE came here]
type Answer = Vec<Vec<Vec<[Option<(i32, DirectionIndex)>; DIRS.len()]>>>;
// loss, streak, current position, direction index FROM WHERE came here
type Priority = BinaryHeap<(Reverse<i32>, Reverse<usize>, Position, DirectionIndex)>;

fn solve<const MIN_STEPS: i32, const MAX_STEPS: i32>(
    lines: &Vec<Vec<i32>>,
    mut ans: Answer,
    mut prio: Priority,
) -> i32 {
    // DAS IST DIJKSTRA !!1!
    while let Some((dist_cur, streak, cur, dir)) = prio.pop() {
        let dist_cur = dist_cur.0;
        let streak = streak.0;

        let dirs = DIRS.iter().enumerate().filter_map(|(ind, next_dir)| {
            // Do not allow to go backward
            if ind == (dir + 2) % DIRS.len() {
                return None;
            }

            if ind == dir && streak == MAX_STEPS as usize {
                return None;
            }

            if ind != dir {
                let to_ = (
                    cur.0 + MIN_STEPS * next_dir.0,
                    cur.1 + MIN_STEPS * next_dir.1,
                );
                // Just return if we are out of the field
                let _ = lines.get(to_.0 as usize)?.get(to_.1 as usize)?;

                let next_loss = (1..=MIN_STEPS)
                    .map(|step| {
                        lines[(cur.0 + step * next_dir.0) as usize]
                            [(cur.1 + step * next_dir.1) as usize]
                    })
                    .sum::<i32>();

                Some((ind, to_, next_loss))
            } else {
                let to_ = (cur.0 + next_dir.0, cur.1 + next_dir.1);
                let next_loss = lines.get(to_.0 as usize)?.get(to_.1 as usize)?;

                Some((ind, to_, *next_loss))
            }
        });

        for (go_next_from_dir_ind, to_, next_loss) in dirs.into_iter() {
            let next_streak = if go_next_from_dir_ind == dir {
                streak + 1
            } else {
                MIN_STEPS as usize
            };

            let mut found_best_case = false;
            for (dist_next, _) in ((MIN_STEPS as usize)..=streak)
                .filter_map(|s| ans[to_.0 as usize][to_.1 as usize][s][go_next_from_dir_ind])
            {
                if dist_cur + next_loss >= dist_next {
                    found_best_case = true;
                    break;
                }
            }

            if !found_best_case {
                ans[to_.0 as usize][to_.1 as usize][next_streak][go_next_from_dir_ind] =
                    Some((dist_cur + next_loss, go_next_from_dir_ind));
                prio.push((
                    Reverse(dist_cur + next_loss),
                    Reverse(next_streak),
                    to_,
                    go_next_from_dir_ind,
                ));
            }
        }
    }

    let ans_ = ans
        .last()
        .expect("Stop kidding me")
        .last()
        .expect("Stop kidding me, dudee")
        .iter()
        .flat_map(|x| x.into_iter())
        .filter_map(|x| *x)
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    ans_.0 - lines[0][0]
}

// 1039
fn part1(file: &str) -> Res<i32> {
    const MIN_STEPS: i32 = 1;
    const MAX_STEPS: i32 = 3;

    let lines: Vec<Vec<i32>> = parse_input(file)?;
    let n = lines.len();
    let m = lines[0].len();

    let mut ans: Answer = vec![vec![vec![[None; 4]; MAX_STEPS as usize + 1]; m]; n];
    let mut prio: Priority = BinaryHeap::new();

    ans[0][0][0][1] = Some((lines[0][0], 1));
    prio.push((Reverse(lines[0][0]), Reverse(0), (0, 0), 1));

    Ok(solve::<MIN_STEPS, MAX_STEPS>(&lines, ans, prio))
}

// 1201
fn part2(file: &str) -> Res<i32> {
    const MIN_STEPS: i32 = 4;
    const MAX_STEPS: i32 = 10;

    let lines = parse_input(file)?;
    let n = lines.len();
    let m = lines[0].len();

    let mut ans: Answer = vec![vec![vec![[None; 4]; MAX_STEPS as usize + 1]; m]; n];
    let mut prio: Priority = BinaryHeap::new();

    let start_loss_1 = (0..=4).map(|i| lines[0][i]).sum::<i32>();
    ans[0][4][4][1] = Some((start_loss_1, 1));
    prio.push((Reverse(start_loss_1), Reverse(4), (0, 4), 1));

    let start_loss_2 = (0..=4).map(|i| lines[i][0]).sum::<i32>();
    ans[4][0][4][2] = Some((start_loss_2, 2));
    prio.push((Reverse(start_loss_2), Reverse(4), (4, 0), 2));

    Ok(solve::<MIN_STEPS, MAX_STEPS>(&lines, ans, prio))
}
