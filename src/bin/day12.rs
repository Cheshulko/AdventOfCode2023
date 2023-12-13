use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day12/in_small.txt";
const IN: &str = "assets/day12/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 7025);
    assert!(ans_part_2 == 11461095383315);

    Ok(())
}

fn parse_input(file: &str) -> Res<Vec<(Vec<u8>, Vec<usize>)>> {
    Ok(read_lines(file)?
        .into_iter()
        .map(|line| {
            let mut line = line.split_whitespace().into_iter();

            let group = line
                .next()
                .expect("Could not parse a group")
                .to_string()
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<_>>();

            let spring_ranges = line
                .next()
                .expect("Could not parse counts")
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();

            (group, spring_ranges)
        })
        .collect::<Vec<_>>())
}

fn is_valid_prefix(pattern: &Vec<u8>, cur_len: usize, to_add_len: usize) -> bool {
    if pattern.len() < cur_len + to_add_len {
        return false;
    }
    for ind in 0..(to_add_len - 1) {
        let pattern_symb = &pattern[cur_len + ind];
        if !((pattern_symb == &b'#') || pattern_symb == &b'?') {
            return false;
        }
    }
    let last = &pattern[cur_len + to_add_len - 1];
    if !((last == &b'.') || (last == &b'?')) {
        return false;
    }
    return true;
}

fn please_find_a_solution(
    cur_len: usize,
    cur_range_index: usize,
    pattern: &Vec<u8>,
    spring_ranges: &Vec<usize>,
    dp: &mut Vec<Vec<Option<u64>>>,
) -> u64 {
    if let Some(ans) = dp[cur_len][cur_range_index] {
        return ans;
    }

    if cur_len == pattern.len() {
        return (cur_range_index == spring_ranges.len()) as u64;
    }

    let mut ans = 0;

    if cur_range_index < spring_ranges.len() {
        let next = spring_ranges[cur_range_index];

        if cur_len + next + 1 <= pattern.len() && is_valid_prefix(pattern, cur_len, next + 1) {
            ans += please_find_a_solution(
                cur_len + next + 1,
                cur_range_index + 1,
                pattern,
                spring_ranges,
                dp,
            );
        }
    }

    if is_valid_prefix(pattern, cur_len, 1) {
        ans += please_find_a_solution(cur_len + 1, cur_range_index, pattern, spring_ranges, dp);
    }

    dp[cur_len][cur_range_index] = Some(ans);
    return ans;
}

// 7025
fn part1(file: &str) -> Res<u64> {
    Ok(parse_input(file)?
        .into_iter()
        .map(|(mut group, spring_ranges)| {
            group.push(b'.');

            let mut dp = vec![vec![None; spring_ranges.len() + 1]; group.len() + 1];

            please_find_a_solution(0, 0, &group, &spring_ranges, &mut dp)
        })
        .sum())
}

// 11461095383315
fn part2(file: &str) -> Res<u64> {
    Ok(parse_input(file)?
        .into_iter()
        .map(|(mut group, mut spring_ranges)| {
            group.push(b'?');
            group = (0..5)
                .flat_map(|_| group.iter().map(|c| *c))
                .collect::<Vec<_>>();

            spring_ranges = (0..5)
                .flat_map(|_| spring_ranges.iter().map(|c| *c))
                .collect::<Vec<_>>();

            *group.last_mut().unwrap() = b'.';
            group.push(b'.');

            let mut dp = vec![vec![None; spring_ranges.len() + 1]; group.len() + 1];

            please_find_a_solution(0, 0, &group, &spring_ranges, &mut dp)
        })
        .sum())
}
