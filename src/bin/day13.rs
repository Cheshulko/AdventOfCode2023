use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day13/in_small.txt";
const IN: &str = "assets/day13/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 30158);
    assert!(ans_part_2 == 36474);

    Ok(())
}

fn parse_input(file: &str) -> Res<Vec<Vec<Vec<u8>>>> {
    let lines = read_lines(file)?;

    let patterns = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let patterns = patterns
        .into_iter()
        .map(|pattern| {
            pattern
                .iter()
                .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(patterns)
}

fn solve(pattern: &Vec<Vec<u8>>, smudges: usize) -> usize {
    let pattern_len = pattern.len();
    let line_len = pattern[0].len();

    let lines_indx = pattern
        .iter()
        .map(|line| {
            let m = line.len();
            let indx = (1..m)
                .map(|i| {
                    let to_left = i;
                    let to_right = m - i;

                    let min = to_left.min(to_right);

                    (0..min)
                        .map(|i_| line[i - i_ - 1] == line[i + i_])
                        .filter(|x| !x)
                        .count()
                })
                .collect::<Vec<_>>();

            indx
        })
        .collect::<Vec<_>>();

    (0..(line_len - 1))
        .filter_map(|line_indx| {
            let smudges_cnt = (0..pattern_len)
                .map(|pattern_index| lines_indx[pattern_index][line_indx])
                .sum::<usize>();

            (smudges_cnt == smudges).then_some(line_indx + 1)
        })
        .next()
        .unwrap_or(0)
}

fn rotate(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    matrix
        .into_iter()
        .enumerate()
        .fold(vec![], |mut v, (i, row)| {
            row.into_iter().enumerate().for_each(|(j, x)| {
                if let Some(v_) = v.get_mut(j) {
                    v_.push(x);
                } else {
                    v.push(vec![x]);
                }
            });
            v
        })
}

// 30158
fn part1(file: &str) -> Res<usize> {
    let patterns = parse_input(file)?;
    let mut ans = 0;

    for pattern in patterns.into_iter() {
        ans += solve(&pattern, 0);
        let pattern = rotate(pattern);
        ans += 100 * solve(&pattern, 0);
    }

    Ok(ans)
}

// 36474
fn part2(file: &str) -> Res<usize> {
    let patterns = parse_input(file)?;
    let mut ans = 0;

    for pattern in patterns.into_iter() {
        ans += solve(&pattern, 1);
        let pattern = rotate(pattern);
        ans += 100 * solve(&pattern, 1);
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::rotate;

    #[test]
    fn rotate_works() {
        let v1 = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let v2 = vec![vec![1, 3, 5, 7], vec![2, 4, 6, 8]];

        let v1 = rotate(v1);

        assert!(v1 == v2)
    }
}
