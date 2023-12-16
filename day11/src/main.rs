use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::{all_consuming, map},
    error::VerboseError,
    multi::{many1, separated_list1},
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let len = input.lines().next().unwrap().len();
    let mut stars = parse(input).unwrap();
    let mut cols: HashSet<_> = (0..len).into_iter().collect();

    let mut row = 0;
    let mut row_off = 0;
    for (x, y) in &mut stars {
        if *y > row {
            row_off += 1;
            row = *y + 1;
        } else if row == *y {
            row = *y + 1;
        }

        *y += row_off;
        cols.remove(x);
    }

    for (x, _) in &mut stars {
        *x = *x + cols.iter().filter(|n| **n < *x).count();
    }

    let mut sum = 0;
    for i in 0..stars.len() {
        for j in (i + 1)..stars.len() {
            sum += stars[i].0.abs_diff(stars[j].0) + stars[i].1.abs_diff(stars[j].1);
        }
    }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let len = input.lines().next().unwrap().len();
    let mut stars = parse(input).unwrap();
    let mut cols: HashSet<_> = (0..len).into_iter().collect();
    let mult = 1_000_000 - 1;

    let mut row = 0;
    let mut row_off = 0;
    for (x, y) in &mut stars {
        if *y > row {
            row_off += 1;
            row = *y + 1;
        } else if row == *y {
            row = *y + 1;
        }

        *y += row_off * mult;
        cols.remove(x);
    }

    for (x, _) in &mut stars {
        *x = *x + cols.iter().filter(|n| **n < *x).count() * mult;
    }

    let mut sum = 0;
    for i in 0..stars.len() {
        for j in (i + 1)..stars.len() {
            sum += stars[i].0.abs_diff(stars[j].0) + stars[i].1.abs_diff(stars[j].1);
        }
    }

    println!("Sum: {sum}");
}

fn parse(input: &str) -> Result<Vec<(usize, usize)>, Err<VerboseError<&str>>> {
    all_consuming(map(
        separated_list1(
            newline,
            map(many1(alt((char('#'), char('.')))), |v| {
                v.into_iter().enumerate().filter(|(_, c)| *c == '#')
            }),
        ),
        |v| {
            v.into_iter()
                .enumerate()
                .flat_map(|(y, l)| l.map(move |(x, _)| (x, y)))
                .collect()
        },
    ))(input)
    .map(|r| r.1)
}
