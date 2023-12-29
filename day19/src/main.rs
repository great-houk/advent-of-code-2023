use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{digit1, one_of},
    combinator::{all_consuming, map, map_res},
    error::VerboseError,
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Err,
};
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

#[derive(Debug, Clone)]
struct InputRange {
    mins: [usize; 4],
    maxs: [usize; 4],
}

impl InputRange {
    pub fn new() -> Self {
        Self {
            mins: [1; 4],
            maxs: [4000; 4],
        }
    }

    pub fn apply(&mut self, quality: char, rule: char, num: usize) {
        let ind = match quality {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Invalid quality {quality}"),
        };
        if rule == '>' {
            self.mins[ind] = (num + 1).max(self.mins[ind]);
        } else if rule == '<' {
            self.maxs[ind] = (num - 1).min(self.maxs[ind]);
        } else {
            panic!("Invalid rule {rule}");
        }
    }

    pub fn apply_inverse(&mut self, quality: char, rule: char, num: usize) {
        let ind = match quality {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Invalid quality {quality}"),
        };
        if rule == '>' {
            self.maxs[ind] = num.min(self.maxs[ind]);
        } else if rule == '<' {
            self.mins[ind] = num.max(self.mins[ind]);
        } else {
            panic!("Invalid rule {rule}");
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..4 {
            if self.mins[i] > self.maxs[i] {
                return false;
            }
        }
        true
    }

    pub fn combos(&self) -> usize {
        let mut ret = 1;
        for i in 0..4 {
            ret *= self.maxs[i] - self.mins[i] + 1;
        }
        ret
    }

    pub fn check_part(&self, part: &[usize; 4]) -> bool {
        for i in 0..4 {
            if part[i] < self.mins[i] || part[i] > self.maxs[i] {
                return false;
            }
        }
        true
    }
}

fn part1(input: &str) {
    let (workflows, parts) = parse(input).unwrap();

    let mut ranges = vec![("in", InputRange::new())];
    let mut successes = vec![];

    while let Some((name, mut range)) = ranges.pop() {
        let (rules, end) = &workflows[name];

        for (quality, rule, num, tag) in rules {
            let mut r = range.clone();
            r.apply(*quality, *rule, *num);
            range.apply_inverse(*quality, *rule, *num);

            if range.is_valid() {
                match *tag {
                    "A" => successes.push(r),
                    "R" => (),
                    _ => ranges.push((tag, r)),
                }
            }
        }

        if range.is_valid() {
            match *end {
                "A" => successes.push(range),
                "R" => (),
                _ => ranges.push((end, range)),
            }
        }
    }

    let sum: usize = parts
        .into_iter()
        .filter(|p| {
            for success in &successes {
                if success.check_part(p) {
                    return true;
                }
            }
            false
        })
        .map(|p| p.iter().sum::<usize>())
        .sum();

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let (workflows, _) = parse(input).unwrap();

    let mut ranges = vec![("in", InputRange::new())];
    let mut successes = vec![];

    while let Some((name, mut range)) = ranges.pop() {
        let (rules, end) = &workflows[name];

        for (quality, rule, num, tag) in rules {
            let mut r = range.clone();
            r.apply(*quality, *rule, *num);
            range.apply_inverse(*quality, *rule, *num);

            if range.is_valid() {
                match *tag {
                    "A" => successes.push(r),
                    "R" => (),
                    _ => ranges.push((tag, r)),
                }
            }
        }

        if range.is_valid() {
            match *end {
                "A" => successes.push(range),
                "R" => (),
                _ => ranges.push((end, range)),
            }
        }
    }

    let sum: usize = successes.iter().map(|r| r.combos()).sum();

    println!("Sum: {sum}");
}

fn parse(
    input: &str,
) -> Result<
    (
        HashMap<&str, (Vec<(char, char, usize, &str)>, &str)>,
        Vec<[usize; 4]>,
    ),
    Err<VerboseError<&str>>,
> {
    all_consuming(separated_pair(
        map(
            separated_list1(
                tag("}\n"),
                map(
                    tuple((
                        terminated(take_till(|c: char| c == '{'), tag("{")),
                        separated_list1(
                            tag(","),
                            tuple((
                                one_of("xmas"),
                                one_of("<>"),
                                map_res(digit1, str::parse),
                                preceded(tag(":"), take_till(|c: char| c == ',')),
                            )),
                        ),
                        preceded(tag(","), take_till(|c: char| c == '}')),
                    )),
                    |(name, rules, end)| (name, (rules, end)),
                ),
            ),
            |v| v.into_iter().collect(),
        ),
        tag("}\n\n"),
        terminated(
            separated_list1(
                tag("}\n"),
                map(
                    count(
                        preceded(
                            take_till(|c: char| c.is_numeric()),
                            map_res(digit1, str::parse),
                        ),
                        4,
                    ),
                    |v| v.try_into().unwrap(),
                ),
            ),
            tag("}"),
        ),
    ))(input)
    .map(|r| r.1)
}
