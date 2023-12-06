use std::collections::VecDeque;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day5/in_small.txt";
const IN: &str = "assets/day5/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    Ok(())
}

type Seeds = Vec<i64>;
type MappingRange = (i64, i64, i64);
type Mapping = (String, String, Vec<MappingRange>);

fn parse_input(file: &str) -> Res<(Seeds, Vec<Mapping>)> {
    let lines = read_lines(file)?;
    let lines = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let mut lines_iter = lines.into_iter();

    let seeds_str = lines_iter.next().expect("Could not parse seeds str");
    let seeds = seeds_str[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|seed| seed.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let mappings = lines_iter
        .map(|line| {
            let mut line_iter = line.into_iter();

            let mut from_to = line_iter
                .next()
                .expect("Could not get From-To pair")
                .split_whitespace()
                .next()
                .and_then(|from_to| {
                    Some(
                        from_to
                            .split("-to-")
                            .map(|str| str.to_string())
                            .collect::<Vec<String>>(),
                    )
                })
                .expect("Could not parse From-To pair");

            let to = from_to.pop().expect("Could not pop `to` value");
            let from = from_to.pop().expect("Could not pop `from` value");

            let mappings = line_iter
                .map(|mapping| {
                    let mapping_range = mapping
                        .split(' ')
                        .map(|str| str.parse::<i64>().expect("Could not parse mapping value"))
                        .collect::<Vec<_>>();

                    (mapping_range[1], mapping_range[0], mapping_range[2])
                })
                .collect::<Vec<_>>();

            (from, to, mappings)
        })
        .collect::<Vec<_>>();

    Ok((seeds, mappings))
}

// 331445006
fn part1(file: &str) -> Res<i64> {
    let (seeds, mappings) = parse_input(file)?;

    Ok(seeds
        .into_iter()
        .map(|mut seed| {
            for (_, _, mapping) in mappings.iter() {
                for (from_mapping, to_mapping, range) in mapping {
                    if *from_mapping <= seed && *from_mapping + *range - 1 >= seed {
                        let diff = seed - *from_mapping;
                        let to = *to_mapping + diff;
                        seed = to;
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .expect("Something went wrong"))
}

// 6472060
fn part2(file: &str) -> Res<i64> {
    let (seeds, mappings) = parse_input(file)?;

    let mut ranges_queue = VecDeque::<(i64, i64, usize)>::new();

    let mut seeds_iter = seeds.into_iter();
    while let (Some(seed_id), Some(seed_range)) = (seeds_iter.next(), seeds_iter.next()) {
        ranges_queue.push_back((seed_id, seed_id + seed_range - 1, 0));
    }

    let mappings_depth = mappings
        .into_iter()
        .fold(vec![], |mut depths, (_, _, cur)| {
            depths.push(cur);
            depths
        });

    let mut ans = i64::MAX;
    while let Some((seed_left, seed_right, depth)) = ranges_queue.pop_front() {
        let left_point_seed = seed_left;
        let right_point_seed = seed_right;

        if depth >= mappings_depth.len() {
            ans = ans.min(left_point_seed);
        } else {
            let mut intersections = vec![];
            for (from_mapping, to_mapping, range) in mappings_depth[depth].iter() {
                let left_point_mapping = *from_mapping;
                let right_point_mapping = *from_mapping + *range - 1;

                // Find inteesection between 2 ranges
                let left_point = left_point_seed.max(left_point_mapping);
                let right_point = right_point_seed.min(right_point_mapping);

                if left_point <= right_point {
                    let diff = *to_mapping - *from_mapping;

                    let left_point_mapped = left_point + diff;
                    let right_point_mapped = right_point + diff;

                    ranges_queue.push_back((left_point_mapped, right_point_mapped, depth + 1));
                    intersections.push((left_point, right_point));
                }
            }

            intersections.sort_unstable();

            if !intersections.is_empty() {
                // Add left tail
                let most_left_intersection = intersections.first().unwrap().0;
                if left_point_seed < most_left_intersection {
                    ranges_queue.push_back((left_point_seed, most_left_intersection, depth + 1));
                }

                // Add right tail
                let most_right_intersection = intersections.last().unwrap().1;
                if right_point_seed > most_right_intersection {
                    ranges_queue.push_back((
                        most_right_intersection + 1,
                        right_point_seed,
                        depth + 1,
                    ));
                }

                // Add range beetwen intersections
                for intersection in intersections.windows(2) {
                    let left_intersection = intersection[0];
                    let right_intersection = intersection[1];
                    if right_intersection.0 - left_intersection.1 > 1 {
                        ranges_queue.push_back((
                            right_intersection.1 + 1,
                            left_intersection.0 - 1,
                            depth + 1,
                        ));
                    }
                }
            } else {
                // Add all inittial range
                ranges_queue.push_back((left_point_seed, right_point_seed, depth + 1));
            }
        }
    }

    Ok(ans)
}
