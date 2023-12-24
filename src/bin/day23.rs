use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day23/in_small.txt";
const IN: &str = "assets/day23/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 2186);
    assert!(ans_part_2 == 6802);

    Ok(())
}

type Direction = (i32, i32);

const UP: Direction = (-1, 0);
const RIGHT: Direction = (0, 1);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);

const DIRS: &[Direction; 4] = &[UP, RIGHT, DOWN, LEFT];

fn parse_input(file: &str) -> Res<Vec<Vec<Type>>> {
    let lines = read_lines(file)?;

    let mtx = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Type::Forest,
                    '.' => Type::Path,
                    '<' => Type::Slope(LEFT),
                    '>' => Type::Slope(RIGHT),
                    'v' => Type::Slope(DOWN),
                    '^' => Type::Slope(UP),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(mtx)
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Forest,
    Path,
    Slope(Direction),
}

impl Type {
    fn next(
        &self,
        (i, j): (usize, usize),
        mtx: &Vec<Vec<Type>>,
        compressed_path: Option<&Vec<Vec<Option<Vec<(usize, usize, i32)>>>>>,
        cares_about_slope: bool,
    ) -> Vec<(usize, usize, i32)> {
        if compressed_path.is_some() {
            if let Some(to) = &(compressed_path.unwrap())[i][j] {
                return to.clone();
            }
        }

        match self {
            Type::Path => DIRS
                .iter()
                .filter_map(|(di, dj)| {
                    let to_i = i as i32 + di;
                    let to_j = j as i32 + dj;
                    let t = mtx.get(to_i as usize)?.get(to_j as usize)?;

                    (t != &Type::Forest).then_some((to_i as usize, to_j as usize, 1))
                })
                .collect::<Vec<_>>(),
            Type::Slope(dir) => {
                if cares_about_slope {
                    let (di, dj) = dir;
                    vec![((i as i32 + di) as usize, (j as i32 + dj) as usize, 1)]
                } else {
                    DIRS.iter()
                        .filter_map(|(di, dj)| {
                            let to_i = i as i32 + di;
                            let to_j = j as i32 + dj;
                            let t = mtx.get(to_i as usize)?.get(to_j as usize)?;

                            (t != &Type::Forest).then_some((to_i as usize, to_j as usize, 1))
                        })
                        .collect::<Vec<_>>()
                }
            }
            Type::Forest => unreachable!(),
        }
    }
}

fn dfs_main(
    (cur_i, cur_j): (usize, usize),
    (target_i, target_j): (usize, usize),
    used: &mut Vec<Vec<bool>>,
    compressed_path: &Vec<Vec<Option<Vec<(usize, usize, i32)>>>>,
    mtx: &Vec<Vec<Type>>,
    cares_about_slope: bool,
) -> i32 {
    if cur_i == target_i && cur_j == target_j {
        return 0;
    }

    used[cur_i][cur_j] = true;

    let mut mx = i32::MIN;
    let tos = mtx[cur_i][cur_j].next(
        (cur_i, cur_j),
        mtx,
        Some(&compressed_path),
        cares_about_slope,
    );

    for (to_i, to_j, dist) in &tos {
        if !used[*to_i][*to_j] {
            mx = mx.max(
                dist + dfs_main(
                    (*to_i, *to_j),
                    (target_i, target_j),
                    used,
                    compressed_path,
                    mtx,
                    cares_about_slope,
                ),
            );
        }
    }

    used[cur_i][cur_j] = false;

    return mx;
}

fn dfs_compress(
    (cur_i, cur_j): (usize, usize),
    target: (usize, usize),
    mtx: &Vec<Vec<Type>>,
    used: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize, i32)> {
    used[cur_i][cur_j] = true;

    let tos = mtx[cur_i][cur_j]
        .next((cur_i, cur_j), mtx, None, false)
        .into_iter()
        .collect::<Vec<_>>();

    let mut compressed_path = if tos.len() > 2 {
        if tos.iter().any(|x| used[x.0][x.1]) {
            vec![(cur_i, cur_j, 1)]
        } else {
            tos.into_iter().fold(vec![], |mut v, (i, j, _)| {
                if !used[i][j] {
                    v.push((i, j, 0));
                }
                v
            })
        }
    } else {
        tos.into_iter()
            .flat_map(|(i, j, _)| {
                if !used[i][j] {
                    dfs_compress((i, j), target, mtx, used)
                } else {
                    vec![]
                }
            })
            .map(|(i, j, dist)| (i, j, dist + 1))
            .collect::<Vec<_>>()
    };

    used[cur_i][cur_j] = false;

    if (cur_i, cur_j) == target {
        compressed_path.push((target.0, target.1, 0));
    }
    compressed_path
}

// 2186
fn part1(file: &str) -> Res<i32> {
    let mtx = parse_input(file)?;

    let n = mtx.len();
    let m = mtx[0].len();

    let mut used = vec![vec![false; m]; n];
    let compressed_path = vec![vec![None; m]; n];

    let ans = dfs_main(
        (0, 1),
        (n - 1, m - 2),
        &mut used,
        &compressed_path,
        &mtx,
        true,
    );

    Ok(ans)
}

// 6802
fn part2(file: &str) -> Res<i32> {
    let mtx = parse_input(file)?;

    let n = mtx.len();
    let m = mtx[0].len();

    let mut used = vec![vec![false; m]; n];
    let mut compressed_path = vec![vec![None; m]; n];

    for i in 0..n {
        for j in 0..m {
            if mtx[i][j] != Type::Forest {
                compressed_path[i][j] = Some(dfs_compress((i, j), (n - 1, m - 2), &mtx, &mut used));
            }
        }
    }

    let ans = dfs_main(
        (0, 1),
        (n - 1, m - 2),
        &mut used,
        &compressed_path,
        &mtx,
        false,
    );

    Ok(ans)
}
