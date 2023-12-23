use std::collections::{HashSet, VecDeque};

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day22/in_small.txt";
const IN: &str = "assets/day22/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 463);
    assert!(ans_part_2 == 89727);

    Ok(())
}

type Point = (usize, usize, usize);

#[derive(Debug)]
struct Cube(Point, Point, usize);

impl Cube {
    fn fall(&mut self, field: &mut Vec<Vec<Vec<i32>>>, mxs: &mut Vec<Vec<usize>>) {
        let lower = self.0 .2;
        let upper = self.1 .2;
        let heigh = upper - lower + 1;

        let mut mx = 0;
        for x in self.0 .0..=self.1 .0 {
            for y in self.0 .1..=self.1 .1 {
                mx = mx.max(mxs[x][y]);
            }
        }
        assert!(lower > mx);
        let down = lower - mx - 1;

        for x in self.0 .0..=self.1 .0 {
            for y in self.0 .1..=self.1 .1 {
                mxs[x][y] = mx + heigh;
            }
        }

        self.0 .2 -= down;
        self.1 .2 -= down;

        for x in self.0 .0..=self.1 .0 {
            for y in self.0 .1..=self.1 .1 {
                for z in self.0 .2..=self.1 .2 {
                    field[x][y][z] = self.2 as i32;
                }
            }
        }
    }
}

fn parse_input(file: &str) -> Res<Vec<Cube>> {
    let lines = read_lines(file)?;

    let cubes = lines
        .into_iter()
        .enumerate()
        .map(|(ind, line)| {
            let line_iter = line.split('~');
            let points = line_iter
                .into_iter()
                .map(|x| {
                    let points = x
                        .split(',')
                        .into_iter()
                        .map(|y| y.parse::<usize>().unwrap())
                        .take(3)
                        .collect::<Vec<_>>();
                    (points[0], points[1], points[2])
                })
                .take(2)
                .collect::<Vec<_>>();
            Cube(points[0], points[1], ind)
        })
        .collect::<Vec<_>>();

    Ok(cubes)
}

fn solve(file: &str) -> Res<(Vec<bool>, Vec<HashSet<usize>>, Vec<HashSet<usize>>)> {
    let mut cubes = parse_input(file)?;

    cubes.sort_by(
        |Cube(a_f, a_s, _), Cube(b_f, b_s, _)| match a_f.2.cmp(&b_f.2) {
            std::cmp::Ordering::Equal => a_s.2.cmp(&b_s.2),
            x @ _ => x,
        },
    );

    let mut max_dim = 0;
    for cube in cubes.iter_mut() {
        max_dim = max_dim.max(cube.1 .0);
        max_dim = max_dim.max(cube.1 .1);
        max_dim = max_dim.max(cube.1 .2);
    }

    let mut field = vec![vec![vec![-1; max_dim]; max_dim]; max_dim];
    let mut mxs = vec![vec![0; max_dim]; max_dim];

    for cube in cubes.iter_mut() {
        cube.fall(&mut field, &mut mxs);
    }

    let mut can_not_be_removed: Vec<bool> = vec![true; cubes.len()];
    let mut support_for: Vec<HashSet<usize>> = vec![HashSet::new(); cubes.len()];
    let mut lays_on: Vec<HashSet<usize>> = vec![HashSet::new(); cubes.len()];

    for cube in cubes.iter().rev() {
        let bottom = cube.0 .2;

        for x in cube.0 .0..=cube.1 .0 {
            for y in cube.0 .1..=cube.1 .1 {
                if field[x][y][bottom - 1] != -1 {
                    lays_on[cube.2].insert(field[x][y][bottom - 1] as usize);
                }
            }
        }

        for x in &lays_on[cube.2] {
            support_for[*x as usize].insert(cube.2);
        }

        if lays_on[cube.2].len() == 1 {
            can_not_be_removed[*lays_on[cube.2].iter().next().unwrap() as usize] = false;
        }
    }

    Ok((can_not_be_removed, support_for, lays_on))
}

// 463
fn part1(file: &str) -> Res<usize> {
    let (can_not_be_removed, _, _) = solve(file)?;

    Ok(can_not_be_removed.into_iter().filter(|x| *x).count())
}

// 89727
fn part2(file: &str) -> Res<usize> {
    let (can_not_be_removed, support_for, lays_on) = solve(file)?;

    let ans = can_not_be_removed
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| (!x).then_some(i))
        .map(|i| {
            let mut falls = HashSet::<usize>::new();
            let mut q = VecDeque::new();
            q.push_back(i);

            while let Some(x) = q.pop_front() {
                falls.insert(x);

                for lays_on_current in &support_for[x] {
                    if falls
                        .intersection(&lays_on[*lays_on_current])
                        .collect::<HashSet<_>>()
                        .len()
                        == lays_on[*lays_on_current].len()
                    {
                        falls.insert(*lays_on_current);
                        q.push_back(*lays_on_current);
                    };
                }
            }

            falls.len() - 1
        })
        .sum::<usize>();

    Ok(ans)
}
