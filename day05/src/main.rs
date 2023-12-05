use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_till1, take_until},
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, eof, flat_map, map, map_parser, map_res, opt},
    error::{context, convert_error, VerboseError},
    multi::{count, many1, many_till, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    Err,
};
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let (mut seeds, steps) = parse(input).unwrap();

    for step in steps {
        for seed in &mut seeds {
            for map in &step {
                if map.0.contains(seed) {
                    *seed = *seed + map.1.start - map.0.start;
                    break;
                }
            }
        }
    }

    println!("Min: {}", seeds.iter().min().unwrap());
}

fn part2(input: &str) {
    let (mut seeds, steps) = parse(input).unwrap();
    seeds = seeds
        .into_iter()
        .tuples()
        .map(|(start, length)| start..(start + length))
        .flatten()
        .collect();

    for step in steps {
        for seed in &mut seeds {
            for map in &step {
                if map.0.contains(seed) {
                    *seed = *seed + map.1.start - map.0.start;
                    break;
                }
            }
        }
    }

    println!("Min: {}", seeds.iter().min().unwrap());
}

fn parse(
    input: &str,
) -> Result<(Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>), Err<VerboseError<&str>>> {
    all_consuming(tuple((
        preceded(
            tag("seeds: "),
            separated_list1(char(' '), map_res(digit1, str::parse)),
        ),
        many1(preceded(
            take_till1(|c: char| c.is_numeric()),
            many1(map(
                count(
                    terminated(map_res(digit1, str::parse), alt((take(1usize), eof))),
                    3,
                ),
                |c| (c[1]..(c[1] + c[2]), c[0]..(c[0] + c[2])),
            )),
        )),
    )))(input)
    .map(|r| r.1)
}
