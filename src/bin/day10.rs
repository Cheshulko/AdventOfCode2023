use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day10/in_small.txt";
const IN: &str = "assets/day10/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 6778);
    assert!(ans_part_2 == 433);

    Ok(())
}

type Matrix = Vec<Vec<[bool; 4]>>;
type UsedMatrix = Vec<Vec<bool>>;
type Point = (i32, i32);

// UP, RIGHT, DOWN, LEFT
const DIRS: &[Point; 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn adjacent_to(dir: usize) -> usize {
    (dir + 2) % DIRS.len()
}

fn parse_input(file: &str) -> Res<(Point, Matrix)> {
    let lines = read_lines(file)?;
    let n = lines.len();
    let m = lines[0].len();

    let mut start = (-1, -1);

    let mut mtx: Matrix = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| match c {
                    '|' => [true, false, true, false],
                    '-' => [false, true, false, true],
                    'L' => [true, true, false, false],
                    'J' => [true, false, false, true],
                    '7' => [false, false, true, true],
                    'F' => [false, true, true, false],
                    '.' => [false, false, false, false],
                    'S' => {
                        start = (i as i32, j as i32);
                        [false, false, false, false]
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    DIRS.iter().enumerate().for_each(|(d, dir)| {
        let to_i = (start.0 as i32 + dir.0) as usize;
        let to_j = (start.1 as i32 + dir.1) as usize;

        mtx[start.0 as usize][start.1 as usize][d] =
            to_i < n && to_j <= m && mtx[to_i][to_j][adjacent_to(d)];
    });

    Ok((start, mtx))
}

fn dfs(
    cur: Point,
    par: Point,
    depth: usize,
    mtx: &Matrix,
    mx: &mut usize,
    used: &mut UsedMatrix,
    path: &mut Vec<Point>,
    path_ans: &mut Vec<Point>,
) {
    used[cur.0 as usize][cur.1 as usize] = true;
    path.push(cur);

    let dirs = DIRS
        .iter()
        .enumerate()
        .filter_map(|(ind_dir, dir)| {
            let try_from = mtx.get(cur.0 as usize)?.get(cur.1 as usize)?[ind_dir];

            let to_i = cur.0 + dir.0;
            let to_j = cur.1 + dir.1;

            let try_to = mtx.get(to_i as usize)?.get(to_j as usize)?;

            if try_from && try_to[adjacent_to(ind_dir)] {
                Some((to_i, to_j))
            } else {
                None
            }
        })
        .filter_map(|(to_i, to_j)| {
            if *used.get(to_i as usize)?.get(to_j as usize)? {
                if !(to_i == par.0 && to_j == par.1) {
                    if depth + 1 > *mx {
                        *mx = depth + 1;
                        *path_ans = path.clone();
                    }
                }
                None
            } else {
                Some((to_i, to_j))
            }
        })
        .collect::<Vec<_>>();

    for (to_i, to_j) in dirs.into_iter() {
        dfs((to_i, to_j), cur, depth + 1, mtx, mx, used, path, path_ans)
    }

    path.pop();
}

// 6778
fn part1(file: &str) -> Res<usize> {
    let (start, mtx) = parse_input(file)?;
    let mut used = vec![vec![false; mtx[0].len()]; mtx.len()];
    let mut path = vec![];
    let mut path_ans = vec![];
    let mut mx = 0;
    dfs(
        start,
        (-1, -1),
        0,
        &mtx,
        &mut mx,
        &mut used,
        &mut path,
        &mut path_ans,
    );

    Ok(mx / 2)
}

// 433
fn part2(file: &str) -> Res<i32> {
    let (start, mtx) = parse_input(file)?;
    let n = mtx.len();
    let m = mtx[0].len();
    let mut used = vec![vec![false; mtx[0].len()]; mtx.len()];
    let mut path = vec![];
    let mut path_ans = vec![];
    let mut mx = 0;
    dfs(
        start,
        (-1, -1),
        0,
        &mtx,
        &mut mx,
        &mut used,
        &mut path,
        &mut path_ans,
    );

    let on_path = path_ans
        .into_iter()
        .fold(vec![vec![false; m]; n], |mut on_path, (i, j)| {
            on_path[i as usize][j as usize] = true;
            on_path
        });

    let mut cnt = 0;

    for (i, row) in mtx.iter().enumerate() {
        let mut enclosed = false;
        let mut in_down = false;
        let mut in_up = false;
        for j in 0..row.len() {
            if on_path[i][j] {
                let crossed = match &mtx[i][j] {
                    /* '.' => */ &[false, false, false, false] => false,
                    /* '|' => */ &[true, false, true, false] => true,
                    /* '-' => */ &[false, true, false, true] => false,
                    /* 'L' => */
                    &[true, true, _, _] => {
                        in_up = true;
                        false
                    }
                    /* 'J' => */
                    &[true, _, _, true] => {
                        if in_up {
                            in_up = false;
                            false
                        } else {
                            assert!(in_down);
                            in_down = false;
                            true
                        }
                    }
                    /* 'F' => */
                    &[_, true, true, _] => {
                        in_down = true;
                        false
                    }
                    /* '7' => */
                    &[_, _, true, true] => {
                        if in_down {
                            in_down = false;
                            false
                        } else {
                            assert!(in_up);
                            in_up = false;
                            true
                        }
                    }

                    _ => unreachable!(),
                };

                enclosed ^= crossed;
            } else {
                cnt += enclosed as i32;
            }
        }
    }

    Ok(cnt)
}
