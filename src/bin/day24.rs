use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day24/in_small.txt";
const IN: &str = "assets/day24/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN_SMALL)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 28266);

    Ok(())
}

mod cm {
    pub fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Option<Vec<f64>> {
        let size = matrix.len();
        assert_eq!(size, matrix[0].len() - 1);

        for i in 0..size - 1 {
            for j in i..size - 1 {
                echelon(matrix, i, j);
            }
        }

        for i in (1..size).rev() {
            eliminate(matrix, i);
        }

        let mut inf = false;
        #[allow(clippy::needless_range_loop)]
        for i in 0..size {
            if matrix[i][i] == 0f64 {
                println!("Infinitely many solutions");
                inf = true;
            }
        }

        if inf {
            return None;
        }

        let mut result: Vec<f64> = vec![0f64; size];
        for i in 0..size {
            result[i] = matrix[i][size] / matrix[i][i];
        }
        Some(result)
    }

    fn echelon(matrix: &mut [Vec<f64>], i: usize, j: usize) {
        let size = matrix.len();
        if matrix[i][i] == 0f64 {
        } else {
            let factor = matrix[j + 1][i] / matrix[i][i];
            (i..size + 1).for_each(|k| {
                matrix[j + 1][k] -= factor * matrix[i][k];
            });
        }
    }

    fn eliminate(matrix: &mut [Vec<f64>], i: usize) {
        let size = matrix.len();
        if matrix[i][i] == 0f64 {
        } else {
            for j in (1..i + 1).rev() {
                let factor = matrix[j - 1][i] / matrix[i][i];
                for k in (0..size + 1).rev() {
                    matrix[j - 1][k] -= factor * matrix[i][k];
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::gaussian_elimination;

        #[test]
        fn test_gauss() {
            let mut matrix: Vec<Vec<f64>> = vec![
                vec![1.0, 2.0, 3.0, 0.0],
                vec![3.0, 4.0, 7.0, 2.0],
                vec![6.0, 5.0, 9.0, 11.0],
            ];
            let result = vec![4.0, 1.0, -2.0];
            assert_eq!(gaussian_elimination(&mut matrix).unwrap(), result);
        }
    }
}

type Point = (i64, i64, i64);

#[derive(Debug)]
struct Ray {
    pub position: Point,
    pub direction: Point,
}

fn parse_input(file: &str) -> Res<Vec<Ray>> {
    let lines = read_lines(file)?;

    Ok(lines
        .into_iter()
        .map(|line| {
            let mut line_iter = line.split('@');

            let position = line_iter.next().unwrap();
            let direction = line_iter.next().unwrap();

            let mut position = position.split(',');
            let position = (
                position.next().unwrap().trim().parse::<i64>().unwrap(),
                position.next().unwrap().trim().parse::<i64>().unwrap(),
                position.next().unwrap().trim().parse::<i64>().unwrap(),
            );

            let mut direction = direction.split(',');
            let direction = (
                direction.next().unwrap().trim().parse::<i64>().unwrap(),
                direction.next().unwrap().trim().parse::<i64>().unwrap(),
                direction.next().unwrap().trim().parse::<i64>().unwrap(),
            );

            Ray {
                position,
                direction,
            }
        })
        .collect::<Vec<_>>())
}

// 28266
fn part1(file: &str) -> Res<u32> {
    let rays = parse_input(file)?;

    let n = rays.len();
    let low = 200000000000000.0;
    let high = 400000000000000.0;

    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let mut matrix = vec![
                vec![
                    rays[i].direction.0 as f64,
                    -rays[j].direction.0 as f64,
                    (rays[j].position.0 - rays[i].position.0) as f64,
                ],
                vec![
                    rays[i].direction.1 as f64,
                    -rays[j].direction.1 as f64,
                    (rays[j].position.1 - rays[i].position.1) as f64,
                ],
            ];
            let xy = cm::gaussian_elimination(&mut matrix);

            if let Some(xy) = xy {
                if xy.iter().all(|x| (x != &f64::NAN && x > &0.0)) {
                    let x = rays[i].position.0 as f64 + rays[i].direction.0 as f64 * xy[0];
                    let y = rays[i].position.1 as f64 + rays[i].direction.1 as f64 * xy[0];

                    if x >= low && x <= high && y >= low && y <= high {
                        ans += 1;
                    }
                } else {
                }
            }
        }
    }

    Ok(ans)
}

fn part2(file: &str) -> Res<u32> {
    Ok(42)
}
