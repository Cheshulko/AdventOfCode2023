use input::read_lines;

mod input;

const IN_SMALL: &str = "assets/day1/in_small.txt";
const IN: &str = "assets/day1/in.txt";

fn main() -> Result<(), std::io::Error> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

// 54667
fn part1(file: &str) -> Result<u32, std::io::Error> {
    Ok(read_lines(file)?
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter_map(|line| {
            let first = line.iter().find(|c| c.is_digit(10))?.to_digit(10)?;
            let last = line.iter().rev().find(|c| c.is_digit(10))?.to_digit(10)?;

            Some(first * 10 + last)
        })
        .reduce(|acc, x| acc + x)
        .unwrap_or(1))
}

// 54203
fn part2(file: &str) -> Result<u32, std::io::Error> {
    const VALID_DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let valid_digits_dir = VALID_DIGITS
        .iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<_>>();

    let valid_digits_rev = VALID_DIGITS
        .iter()
        .map(|digit| digit.to_string().chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    fn get_digit_str(digits: &Vec<String>, line: &String) -> Option<(usize, usize)> {
        // pos, digit
        let mut digits = digits
            .iter()
            .enumerate()
            .filter_map(|(ind, digit)| Some((line.find(digit)?, ind + 1)))
            .collect::<Vec<_>>();

        digits.sort_unstable();
        digits.get(0).map(|x| *x)
    }

    fn get_digit(line: &String) -> Option<(usize, usize)> {
        let pos = line.chars().position(|c| c.is_digit(10))?;
        Some((pos, line.chars().nth(pos)?.to_digit(10)? as usize))
    }

    Ok(read_lines(file)?
        .into_iter()
        .filter_map(|line| {
            let line_rev = line.chars().rev().collect::<String>();
            let mut digits = vec![
                get_digit_str(&valid_digits_dir, &line),
                get_digit_str(&valid_digits_rev, &line_rev)
                    .map(|x| (line_rev.len() - 1 - x.0, x.1)),
                get_digit(&line),
                get_digit(&line_rev).map(|x| (line_rev.len() - 1 - x.0, x.1)),
            ]
            .into_iter()
            .filter(|x| x.is_some())
            .collect::<Vec<_>>();

            digits.sort_unstable();
            if let (Some(first), Some(last)) = (digits.first()?, digits.last()?) {
                Some(first.1 * 10 + last.1)
            } else {
                None
            }
        })
        .reduce(|acc, x| acc + x)
        .unwrap_or(1) as u32)
}
