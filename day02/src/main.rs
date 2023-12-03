use std::cmp::max;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    Err,
};

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let maxes = (13, 14, 12);
    let mut sum = 0;

    for (id, rounds) in parse(input).unwrap() {
        if rounds.into_iter().fold(true, |res, (a, b, c)| {
            res && (a <= maxes.0 && b <= maxes.1 && c <= maxes.2)
        }) {
            sum += id;
        }
    }

    println!("Sum is {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;

    for (_, rounds) in parse(input).unwrap() {
        let (a, b, c) = rounds.into_iter().fold((0, 0, 0), |(d, e, f), (a, b, c)| {
            (max(a, d), max(b, e), max(c, f))
        });
        sum += a * b * c;
    }

    println!("Sum is {sum}");
}

fn parse(input: &str) -> Result<Vec<(u32, Vec<(u32, u32, u32)>)>, Err<()>> {
    //Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    separated_list0(
        newline,
        tuple((
            preceded(tag("Game "), map_res(digit1, str::parse)), //
            preceded(
                tag(": "),
                separated_list0(
                    tag("; "),
                    map(
                        separated_list0(
                            tag(", "),
                            alt((
                                terminated(
                                    map(digit1, |num| (str::parse(num).unwrap(), 0, 0)),
                                    tag(" green"),
                                ),
                                terminated(
                                    map(digit1, |num| (0, str::parse(num).unwrap(), 0)),
                                    tag(" blue"),
                                ),
                                terminated(
                                    map(digit1, |num| (0, 0, str::parse(num).unwrap())),
                                    tag(" red"),
                                ),
                            )),
                        ),
                        |l| {
                            l.iter()
                                .fold((0, 0, 0), |(a, b, c), (d, e, f)| (a + d, b + e, c + f))
                        },
                    ),
                ),
            ),
        )),
    )(input)
    .map(|r| r.1)
}
