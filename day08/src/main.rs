use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{take, take_till},
    character::complete::{char, newline},
    combinator::{all_consuming, map},
    error::VerboseError,
    multi::{many1, separated_list1},
    sequence::tuple,
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn lcm(nums: &[u128]) -> u128 {
    let mut lcm = nums[0];
    for i in 1..nums.len() {
        let mut a = lcm;
        let mut b = nums[i];
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        lcm *= nums[i] / a;
    }

    lcm
}

fn part1(input: &str) {
    let (dirs, map) = parse(input).unwrap();
    let dirs = dirs.into_iter().cycle();

    let mut curr = "AAA";
    let mut count = 0;
    for dir in dirs {
        if curr == "ZZZ" {
            break;
        }

        curr = match dir {
            Dir::Left => map[curr].0,
            Dir::Right => map[curr].1,
        };
        count += 1;
    }

    println!("Count: {count}");
}

fn part2(input: &str) {
    let (dirs, map) = parse(input).unwrap();
    let dirs = dirs.into_iter().enumerate().cycle();

    let mut currs: Vec<_> = map
        .keys()
        .cloned()
        .filter(|s| s.as_bytes()[2] == 'A' as u8)
        .map(|s| (s, HashMap::new(), None))
        .collect();

    let mut count = 0u128;
    for (i, dir) in dirs {
        let mut done = true;
        for (curr, zees, loops) in &mut currs {
            if loops.is_some() {
                continue;
            }

            if curr.as_bytes()[2] == 'Z' as u8 {
                if let Some(c) = zees.get(&(i, *curr)) {
                    *loops = Some((count - c, (i, *curr)));
                    continue;
                } else {
                    zees.insert((i, *curr), count);
                }
            }

            *curr = match dir {
                Dir::Left => map[*curr].0,
                Dir::Right => map[*curr].1,
            };
            done = false;
        }
        if done {
            break;
        }

        count += 1;
    }

    let cycles: Vec<_> = currs
        .into_iter()
        .map(|(_, _, l)| {
            let l = l.unwrap();
            l.0
        })
        .collect();

    let max = lcm(&cycles);

    println!("Count: {max}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn parse(input: &str) -> Result<(Vec<Dir>, HashMap<&str, (&str, &str)>), Err<VerboseError<&str>>> {
    all_consuming(tuple((
        many1(alt((
            map(char('L'), |_| Dir::Left),
            map(char('R'), |_| Dir::Right),
        ))),
        take_till(|c: char| c.is_alphanumeric()),
        map(
            separated_list1(
                newline,
                map(
                    tuple((
                        take(3usize), // "AAA"
                        take(4usize), // " = ("
                        take(3usize), // "BBB"
                        take(2usize), // ", "
                        take(3usize), // "CCC"
                        take(1usize), // ")"
                    )),
                    |(name, _, left, _, right, _)| (name, (left, right)),
                ),
            ),
            |v| v.into_iter().collect(),
        ),
    )))(input)
    .map(|(_, r)| (r.0, r.2))
}
