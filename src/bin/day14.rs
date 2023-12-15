use std::collections::{HashSet, VecDeque};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day14/in_small.txt";
const IN: &str = "assets/day14/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 102497);
    assert!(ans_part_2 == 105008);

    Ok(())
}

fn parse_input(file: &str) -> Res<VecDeque<VecDeque<u8>>> {
    let lines = read_lines(file)?;

    let lines = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect::<VecDeque<_>>();

    Ok(lines)
}

fn rotate(matrix: VecDeque<VecDeque<u8>>) -> VecDeque<VecDeque<u8>> {
    let n = matrix.len();
    matrix
        .into_iter()
        .enumerate()
        .fold(vec![].into(), |mut v, (i, row)| {
            row.into_iter().enumerate().for_each(|(j, x)| {
                if let Some(v_) = v.get_mut(j) {
                    v_[n - 1 - i] = x;
                } else {
                    let mut row_: VecDeque<u8> = vec![0; n].into();
                    row_[n - 1 - i] = x;
                    v.push_back(row_.into());
                }
            });
            v
        })
}

fn perform(mut lines: VecDeque<VecDeque<u8>>, tilt: bool) -> (VecDeque<VecDeque<u8>>, usize) {
    let mut ans = 0;

    let n = lines.len();
    let m = lines[0].len();

    for j in 0..m {
        let vertical_line = (0..n).map(|i| lines[i][j]).collect::<Vec<_>>();

        let resh = vertical_line
            .iter()
            .enumerate()
            .filter_map(|(ind, c)| (c == &b'#').then_some(ind))
            .collect::<Vec<_>>();

        let _ = resh.windows(2).rev().for_each(|r| {
            let top = r[0];
            let bot = r[1];

            let mut count = vertical_line[(top + 1)..bot]
                .iter()
                .filter(|c| *c == &b'O')
                .count();

            if tilt {
                for i in (top + 1)..bot {
                    lines[i][j] = if count > 0 {
                        count -= 1;
                        b'O'
                    } else {
                        b'.'
                    };
                }
            }

            ans += ((top + 1)..bot)
                .filter_map(|i| (lines[i][j] == b'O').then_some(n - 1 - i))
                .sum::<usize>();
        });
    }
    (lines, ans)
}

fn cycle(mut lines: VecDeque<VecDeque<u8>>) -> VecDeque<VecDeque<u8>> {
    for _ in 0..4 {
        (lines, _) = perform(lines, true);
        lines = rotate(lines);
    }

    return lines;
}

// 102497
fn part1(file: &str) -> Res<usize> {
    let mut lines = parse_input(file)?;

    // Add additional top and bottom '#' to perform a tilt
    let m = lines[0].len();
    lines.push_front(vec![b'#'; m].into());
    lines.push_back(vec![b'#'; m].into());

    for line in &mut lines {
        line.push_front(b'#');
        line.push_back(b'#');
    }

    let (_, ans) = perform(lines, true);

    Ok(ans)
}

// 105008
fn part2(file: &str) -> Res<usize> {
    let mut lines: VecDeque<VecDeque<u8>> = parse_input(file)?;

    // Add additional top and bottom, left and right '#' to perform a tilt with rotation
    let m = lines[0].len();
    lines.push_front(vec![b'#'; m].into());
    lines.push_back(vec![b'#'; m].into());
    for line in &mut lines {
        line.push_front(b'#');
        line.push_back(b'#');
    }

    let mut ans = 0;

    let mut first_already_seen_o_vector: Option<(i32, Vec<(usize, usize)>)> = None;
    let mut seen_o_vectors = HashSet::<Vec<(usize, usize)>>::new();

    let many_many = 1_000_000_000;
    let mut current_iteration = 0;

    while current_iteration < many_many {
        lines = cycle(lines);
        (lines, ans) = perform(lines, false);

        let o_vector = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(j, x)| (x == &b'O').then_some((i, j)))
            })
            .collect::<Vec<_>>();

        if let Some((seen_iteration, first_already_seen_o_vector)) = &first_already_seen_o_vector {
            if *first_already_seen_o_vector == o_vector {
                // Fount a cycle!!1!
                let cycle_length = current_iteration - seen_iteration;
                let new_index = (((many_many - 1) - seen_iteration) / cycle_length) * cycle_length
                    + seen_iteration
                    + 1;
                // Jump to {new_index}
                current_iteration = new_index;
                continue;
            }
        }

        // Hoping to find a cycle
        if seen_o_vectors.contains(&o_vector) && first_already_seen_o_vector.is_none() {
            first_already_seen_o_vector = Some((current_iteration, o_vector.clone()));
        } else {
            seen_o_vectors.insert(o_vector);
        }
        current_iteration += 1;
    }

    Ok(ans)
}
