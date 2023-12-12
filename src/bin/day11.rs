use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day11/in_small.txt";
const IN: &str = "assets/day11/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 9684228);
    assert!(ans_part_2 == 483844716556);

    Ok(())
}

fn parse_input(file: &str) -> Res<(Vec<(usize, usize)>, Vec<usize>, Vec<usize>)> {
    let lines = read_lines(file)?;
    let lines = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = lines.len();
    let m = lines[0].len();

    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    let mut galaxies = vec![];

    for i in 0..n {
        if (0..m).map(|j| &lines[i][j]).all(|c| c == &b'.') {
            expanded_rows.push(i);
        }
    }

    for j in 0..m {
        if (0..n).map(|i| &lines[i][j]).all(|c| c == &b'.') {
            expanded_cols.push(j);
        }
    }

    for (i, line) in lines.iter().enumerate() {
        let galaxies_ =
            line.iter().enumerate().filter_map(
                |(j, c)| {
                    if c == &b'#' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            );
        galaxies.extend(galaxies_);
    }

    Ok((galaxies, expanded_rows, expanded_cols))
}

fn solve(file: &str, k: u64) -> Res<u64> {
    let (galaxies, expanded_rows, expanded_cols) = parse_input(file)?;

    Ok(galaxies
        .iter()
        .enumerate()
        .map(|(galaxy_ind_1, galaxy_1)| {
            galaxies
                .iter()
                .skip(galaxy_ind_1)
                .map(|galaxy_2| {
                    let (i_min, i_max) = (galaxy_1.0.min(galaxy_2.0), galaxy_1.0.max(galaxy_2.0));

                    let er_left = expanded_rows.binary_search(&i_min).err().unwrap();
                    let er_right = expanded_rows.binary_search(&i_max).err().unwrap();

                    let (j_min, j_max) = (galaxy_1.1.min(galaxy_2.1), galaxy_1.1.max(galaxy_2.1));

                    let ec_left = expanded_cols.binary_search(&j_min).err().unwrap();
                    let ec_right = expanded_cols.binary_search(&j_max).err().unwrap();

                    (i_max - i_min) as u64
                        + (k - 1) * (er_right - er_left) as u64
                        + (j_max - j_min) as u64
                        + (k - 1) * (ec_right - ec_left) as u64
                })
                .sum::<u64>()
        })
        .sum::<u64>())
}

// 9684228
fn part1(file: &str) -> Res<u64> {
    solve(file, 2)
}

// 483844716556
fn part2(file: &str) -> Res<u64> {
    solve(file, 1000000)
}
