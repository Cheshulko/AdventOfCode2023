use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day15/in_small.txt";
const IN: &str = "assets/day15/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 506891);
    assert!(ans_part_2 == 230462);

    Ok(())
}

fn parse_input(file: &str) -> Res<Vec<String>> {
    let lines = read_lines(file)?;

    let input = lines.into_iter().next().expect("What ???");

    let steps = input
        .split(',')
        .map(|step| step.to_owned())
        .collect::<Vec<_>>();

    Ok(steps)
}

// 506891
fn part1(file: &str) -> Res<u32> {
    Ok(parse_input(file)?
        .into_iter()
        .map(|step| {
            step.chars().fold(0, |hash, c| {
                let c = (c as u8) as u32;
                17 * (hash + c) % 256
            })
        })
        .sum::<u32>())
}

// 230462
fn part2(file: &str) -> Res<usize> {
    fn modify(
        boxes: &mut Vec<Vec<Option<(String, usize)>>>,
        hash: usize,
        label: &String,
        value: Option<(String, usize)>,
    ) -> bool {
        for len in &mut boxes[hash] {
            if let Some(len_) = len {
                if &len_.0 == label {
                    *len = value;
                    return true;
                }
            }
        }

        return false;
    }

    let steps = parse_input(file)?;
    let mut boxes = vec![Vec::<Option<(String, usize)>>::new(); 256];

    steps.into_iter().for_each(|step| {
        let mut step = step.split(['-', '=']);

        let label = step.next().expect("No label").to_string();
        let focal = step.next().expect("No no no");

        let hash = label.chars().fold(0, |hash, c| {
            let c = (c as u8) as usize;
            17 * (hash + c) % 256
        }) as usize;

        if !focal.is_empty() {
            let focal = focal.parse::<usize>().unwrap();

            if !modify(&mut boxes, hash, &label, Some((label.clone(), focal))) {
                boxes[hash].push(Some((label, focal)));
            }
        } else {
            let _ = modify(&mut boxes, hash, &label, None);
        }
    });

    let mut ans = 0;
    for (box_ind, box_) in boxes.into_iter().enumerate() {
        let box_ = box_.into_iter().filter_map(|len| len).collect::<Vec<_>>();
        for (len_ind, len) in box_.into_iter().enumerate() {
            ans += (1 + box_ind) * (1 + len_ind) * len.1;
        }
    }

    Ok(ans)
}
