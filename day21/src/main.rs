use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    character::complete::{anychar, digit1},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Err,
};
use std::collections::HashMap;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut jobs = parse_input(input).unwrap();

    while let Job::Equation { .. } = jobs["root"] {
        for (name, job) in jobs.clone() {
            if let Job::Equation { left, op, right } = job {
                if let Job::Number(l) = jobs[left] {
                    if let Job::Number(r) = jobs[right] {
                        jobs.insert(name, Job::Number(op.apply(l, r)));
                    }
                }
            }
        }
    }

    dbg!(jobs["root"]);
}

fn part2(input: Str) {
    let mut jobs = parse_input(input).unwrap();
    if let Job::Equation { left, right, .. } = jobs["root"] {
        jobs.insert("root", Job::Root { left, right });
    }
    jobs.insert("humn", Job::Human);

    let mut progress = true;
    while progress {
        progress = false;
        for (name, job) in jobs.clone() {
            // Simplify Down
            if let Job::Equation { left, op, right } = job {
                if let Job::Number(l) = jobs[left] {
                    if let Job::Number(r) = jobs[right] {
                        jobs.insert(name, Job::Number(op.apply(l, r)));
                        progress = true;
                    }
                }
            }
        }
    }

    let (target, name) = {
        let Job::Root { left, right } = jobs["root"] else {unreachable!()};
        if let Job::Equation { .. } = jobs[left] {
            let Job::Number(t) = jobs[right] else {unreachable!()};
            (t, left)
        } else {
            let Job::Number(t) = jobs[right] else {unreachable!()};
            (t, right)
        }
    };

    let mut jobs0 = jobs.clone();
    jobs0.insert("humn", Job::Number(0.));
    let mut jobs1 = jobs.clone();
    jobs1.insert("humn", Job::Number(1_000_000_000_000_000.));

    while let Job::Equation { .. } = jobs0[name] {
        for (name, job) in jobs0.clone() {
            if let Job::Equation { left, op, right } = job {
                if let Job::Number(l) = jobs0[left] {
                    if let Job::Number(r) = jobs0[right] {
                        jobs0.insert(name, Job::Number(op.apply(l, r)));
                    }
                }
            }
        }
    }
    let Job::Number(n0) = jobs0[name] else {unreachable!()};

    while let Job::Equation { .. } = jobs1[name] {
        for (name, job) in jobs1.clone() {
            if let Job::Equation { left, op, right } = job {
                if let Job::Number(l) = jobs1[left] {
                    if let Job::Number(r) = jobs1[right] {
                        jobs1.insert(name, Job::Number(op.apply(l, r)));
                    }
                }
            }
        }
    }
    let Job::Number(n1) = jobs1[name] else {unreachable!()};

    let yell = 1_000_000_000_000_000. * (target - n0) / (n1 - n0);

    dbg!(yell);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Job {
    Number(f64),
    Equation {
        left: Str,
        op: Operation,
        right: Str,
    },
    Human,
    Root {
        left: Str,
        right: Str,
    },
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operation {
    Add,
    Mult,
    Divide,
    Sub,
}

impl Operation {
    fn apply(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Mult => left * right,
            Self::Divide => left / right,
            Self::Sub => left - right,
        }
    }
}

fn parse_input(input: Str) -> Result<HashMap<Str, Job>, Err<Error<Str>>> {
    let (_, v) = all_consuming(separated_list1(
        newline,
        tuple((
            terminated(take(4u8), tag(": ")),
            alt((
                map(digit1, |n: Str| {
                    let n = n.parse().unwrap();
                    Job::Number(n)
                }),
                map(
                    tuple((take(4u8), tag(" "), anychar, tag(" "), take(4u8))),
                    |(left, _, op, _, right)| {
                        let op = match op {
                            '+' => Operation::Add,
                            '-' => Operation::Sub,
                            '/' => Operation::Divide,
                            '*' => Operation::Mult,
                            _ => unreachable!("WOah!"),
                        };
                        Job::Equation { left, op, right }
                    },
                ),
            )),
        )),
    ))(input)?;

    Ok(v.into_iter().collect())
}
