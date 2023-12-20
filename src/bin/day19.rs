use std::collections::HashMap;

use input::{read_lines, Res};

mod input;

const IN_SMALL: &str = "assets/day19/in_small.txt";
const IN: &str = "assets/day19/in.txt";

fn main() -> Res<()> {
    let ans_part_1 = part1(IN)?;
    let ans_part_2 = part2(IN)?;

    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}");

    assert!(ans_part_1 == 346230);
    assert!(ans_part_2 == 124693661917133);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Part {
    X,
    M,
    A,
    S,
}

impl From<&str> for Part {
    fn from(str: &str) -> Self {
        assert!(str.len() == 1);

        match str {
            "x" => Part::X,
            "m" => Part::M,
            "a" => Part::A,
            "s" => Part::S,
            _ => unreachable!(),
        }
    }
}

impl Part {
    fn indx(&self) -> usize {
        match &self {
            Part::X => 0,
            Part::M => 1,
            Part::A => 2,
            Part::S => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum RuleType {
    Le,
    Ge,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rule {
    pub t: RuleType,
    pub part: Part,
    pub value: i128,
    pub next: String,
}

impl Rule {
    fn reverse(&self) -> Self {
        match self.t {
            RuleType::Le => Rule {
                value: self.value - 1,
                t: RuleType::Ge,
                next: self.next.clone(),
                ..*self
            },
            RuleType::Ge => Rule {
                value: self.value + 1,
                t: RuleType::Le,
                next: self.next.clone(),
                ..*self
            },
        }
    }

    fn indx(&self) -> usize {
        self.part.indx()
    }
}

#[derive(Debug, Clone)]
struct Range {
    pub left: i128,
    pub right: i128,
}

impl Range {
    fn apply_rule(&self, rule: &Rule) -> Option<Self> {
        match rule.t {
            RuleType::Le => {
                let right = self.right.min(rule.value - 1);
                (self.left <= right).then_some(Range {
                    right: right,
                    ..*self
                })
            }
            RuleType::Ge => {
                let left = self.left.max(rule.value + 1);
                (left <= self.right).then_some(Range {
                    left: left,
                    ..*self
                })
            }
        }
    }
}

impl Rule {
    fn next(&self, item: &HashMap<Part, i128>) -> Option<String> {
        let item = item.get(&self.part).unwrap();

        return match &self.t {
            RuleType::Le if item < &self.value => Some(self.next.clone()),
            RuleType::Ge if item > &self.value => Some(self.next.clone()),

            _ => None,
        };
    }
}

fn parse_input(
    file: &str,
) -> Res<(
    HashMap<String, (Vec<Rule>, String)>,
    Vec<HashMap<Part, i128>>,
)> {
    let lines = read_lines(file)?;

    let mut block_iter = lines.split(|x| x.is_empty());

    let mut rules: HashMap<String, (Vec<Rule>, String)> = HashMap::new();

    let rules_iter = block_iter.next().unwrap();
    for rule in rules_iter.iter() {
        let mut rule_iter = rule.split(&['{', '}']);
        let name = rule_iter.next().unwrap();
        let rule = rule_iter.next().unwrap();

        let mut list = vec![];
        let mut out = String::new();

        for rule in rule.split(',') {
            let mut rule_iter = rule.split([':']);

            let def = rule_iter.next().unwrap();

            if let Some(to) = rule_iter.next() {
                if def.contains('<') {
                    let mut def_iter = def.split('<');
                    let part = def_iter.next().unwrap().into();
                    let value = def_iter.next().unwrap().parse::<i128>().unwrap();

                    list.push(Rule {
                        part,
                        value,
                        next: to.to_string(),
                        t: RuleType::Le,
                    });
                }
                if def.contains('>') {
                    let mut def_iter = def.split('>');
                    let part = def_iter.next().unwrap().into();
                    let value = def_iter.next().unwrap().parse::<i128>().unwrap();

                    list.push(Rule {
                        part,
                        value,
                        next: to.to_string(),
                        t: RuleType::Ge,
                    });
                }
            } else {
                out = def.to_string();
            }
        }

        rules.insert(name.to_string(), (list, out));
    }

    let mut items = vec![];

    let items_str = block_iter.next().unwrap();
    for item in items_str {
        let mut item_parts = HashMap::new();

        let item = item.trim_matches(['{', '}'].as_slice());
        for part in item.split(',') {
            let mut item_parts_iter = part.split('=');
            let part: Part = item_parts_iter.next().unwrap().into();
            let value = item_parts_iter.next().unwrap().parse::<i128>().unwrap();

            item_parts.insert(part, value);
        }

        items.push(item_parts);
    }

    Ok((rules, items))
}

// 346230
fn part1(file: &str) -> Res<i128> {
    let (rules, items) = parse_input(file)?;

    fn dfs(
        cur: &String,
        rules: &HashMap<String, (Vec<Rule>, String)>,
        item: &HashMap<Part, i128>,
    ) -> bool {
        if let Some((rule, out)) = rules.get(cur) {
            for rule_item in rule {
                if let Some(next) = rule_item.next(item) {
                    return dfs(&next, rules, item);
                }
            }
            return dfs(&out, rules, item);
        } else {
            return cur == "A";
        }
    }

    Ok(items.into_iter().fold(0, |ans, item| {
        ans + if dfs(&"in".to_string(), &rules, &item) {
            item.iter().map(|(_, value)| value).sum::<i128>()
        } else {
            0
        }
    }))
}

// 124693661917133
fn part2(file: &str) -> Res<i128> {
    let (rules, _) = parse_input(file)?;

    let mut dp = HashMap::<(String, Rule), i128>::new();

    fn dfs(
        cur: &String,
        rules: &HashMap<String, (Vec<Rule>, String)>,
        dp: &mut HashMap<(String, Rule), i128>,
        ranges: Vec<Range>,
    ) -> i128 {
        let mut variants = ranges
            .iter()
            .fold(1, |pr, range| pr * (range.right - range.left + 1));

        if let Some((rule, out)) = rules.get(cur) {
            let mut ans = 0;

            let mut cur_ranges = ranges.clone();

            for rule_item in rule {
                let rule_item_index = rule_item.indx();
                let range = &cur_ranges[rule_item_index];

                if let Some(upd_range) = range.apply_rule(rule_item) {
                    let mut new_limits = cur_ranges.clone();
                    new_limits[rule_item_index] = upd_range;
                    let to_add = dfs(&rule_item.next, rules, dp, new_limits);
                    ans += to_add;
                }

                if let Some(rev_upd_limit) = range.apply_rule(&rule_item.reverse()) {
                    variants = variants / (range.right - range.left + 1)
                        * (rev_upd_limit.right - rev_upd_limit.left + 1);

                    cur_ranges[rule_item_index] = rev_upd_limit;
                } else {
                    unreachable!()
                }
            }

            // Variants for `out` (last)
            ans += dfs(out, rules, dp, cur_ranges);

            return ans;
        } else {
            return variants * ((cur == "A") as i128);
        }
    }

    let ranges: Vec<Range> = vec![
        // part: Part::X,
        Range {
            left: 1,
            right: 4000,
        },
        // part: Part::M,
        Range {
            left: 1,
            right: 4000,
        },
        // part: Part::A,
        Range {
            left: 1,
            right: 4000,
        },
        // part: Part::S,
        Range {
            left: 1,
            right: 4000,
        },
    ];

    Ok(dfs(&"in".to_string(), &rules, &mut dp, ranges))
}
