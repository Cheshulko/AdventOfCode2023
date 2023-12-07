use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day6/in_small.txt";
const IN: &str = "assets/day6/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

fn parse_input(file: &str) -> Res<(Vec<i64>, Vec<i64>)> {
    let lines = read_lines(file)?;
    let mut lines_iter = lines.into_iter();

    let time = lines_iter.next().unwrap();
    let distance = lines_iter.next().unwrap();

    let time = time
        .split(':')
        .skip(1)
        .flat_map(|str| str.split(' '))
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect::<Vec<_>>();

    let distance = distance
        .split(':')
        .skip(1)
        .flat_map(|str| str.split(' '))
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect::<Vec<_>>();

    Ok((time, distance))
}

// 211904
fn part1(file: &str) -> Res<i64> {
    let (time, distance) = parse_input(file)?;

    Ok(time
        .into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| (0..=time).filter(|t| (time - t) * t > distance).count() as i64)
        .product::<i64>())
}

// 43364472
fn part2(file: &str) -> Res<i64> {
    let (time, distance) = parse_input(file)?;

    let time = time
        .into_iter()
        .fold(String::new(), |mut str, s| {
            str.extend(s.to_string().chars());
            str
        })
        .parse::<i64>()
        .unwrap();

    let distance = distance
        .into_iter()
        .fold(String::new(), |mut str, s| {
            str.extend(s.to_string().chars());
            str
        })
        .parse::<i64>()
        .unwrap();

    /*
        x * (time - x) >= distance
        -x*x + time*x - distance >= 0
        x*x - time*x + distance <= 0
    */

    let sqrt = ((time * time - 4 * distance) as f64).sqrt();

    let left = (time as f64 - sqrt) / 2.;
    let right = (time as f64 + sqrt) / 2.;

    let left = left.ceil();
    let right = right.floor();

    let ans = (right - left) as i64 + 1;

    Ok(ans)
}
