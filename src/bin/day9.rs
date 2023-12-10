use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day9/in_small.txt";
const IN: &str = "assets/day9/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 1584748274);
    assert!(ans_part_2 == 1026);

    Ok(())
}

fn parse_input(file: &str) -> Res<Vec<Vec<i64>>> {
    let lines = read_lines(file)?;
    let lines = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(lines)
}

// 1584748274
fn part1(file: &str) -> Res<i64> {
    let lines = parse_input(file)?;

    Ok(lines.into_iter().fold(0, |ans, mut line| {
        let mut tail = vec![];

        while !line.iter().all(|x| x == &0) {
            tail.push(
                *line
                    .last()
                    .expect("Could not get a value. Is the list empty?"),
            );
            line = line.windows(2).map(|x| x[1] - x[0]).collect();
        }

        tail.reverse();
        let right_most: Vec<i64> = tail.into_iter().fold(vec![], |mut v, x| {
            let next = v.last().unwrap_or(&0) + x;
            v.push(next);
            v
        });

        ans + right_most.last().unwrap_or(&0)
    }))
}

// 1026
fn part2(file: &str) -> Res<i64> {
    let lines = parse_input(file)?;

    Ok(lines.into_iter().fold(0, |ans, mut line| {
        let mut head = vec![];

        while !line.iter().all(|x| x == &0) {
            head.push(
                *line
                    .first()
                    .expect("Could not get a value. Is the list empty?"),
            );

            line = line.windows(2).map(|x| x[1] - x[0]).collect();
        }

        head.reverse();
        let left_most = head.into_iter().fold(vec![], |mut v, x| {
            let next = x - v.last().unwrap_or(&0);
            v.push(next);
            v
        });

        ans + left_most.last().unwrap_or(&0)
    }))
}
