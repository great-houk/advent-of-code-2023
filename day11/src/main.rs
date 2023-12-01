use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::tuple,
    IResult,
};
use std::collections::VecDeque;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .map(Result::unwrap)
        .map(|(_, m)| m)
        .collect_vec();
    let mut inspections = vec![0; monkeys.len()];

    const ROUNDS: i32 = 20;

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                // Add to inspections
                inspections[i] += 1;
                // Inspect
                match monkeys[i].operation {
                    Operation::Mult(num) => item *= num,
                    Operation::Plus(num) => item += num,
                    Operation::Square => item *= item,
                };
                // Put down
                item /= 3;
                // Test
                let bool = item % monkeys[i].test == 0;
                // Throw
                let ind = if bool {
                    monkeys[i].throw_to.0
                } else {
                    monkeys[i].throw_to.1
                };
                monkeys[ind].items.push_back(item);
            }
        }
    }

    inspections.sort();
    let (m1, m2) = inspections.iter().rev().take(2).collect_tuple().unwrap();
    println!("Monkey business: {}", m1 * m2);
}

fn part2(input: Str) {
    let mut monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .map(Result::unwrap)
        .map(|(_, m)| m)
        .collect_vec();
    let mut inspections = vec![0usize; monkeys.len()];
    let lcm = monkeys.iter().map(|m| m.test).fold(1, |a, e| a * e);

    const ROUNDS: i32 = 10_000;

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                // Add to inspections
                inspections[i] += 1;
                // Inspect
                match monkeys[i].operation {
                    Operation::Mult(num) => item *= num,
                    Operation::Plus(num) => item += num,
                    Operation::Square => item *= item,
                };
                // Smallerize
                item %= lcm;
                // Test
                let bool = item % monkeys[i].test == 0;
                // Throw
                let ind = if bool {
                    monkeys[i].throw_to.0
                } else {
                    monkeys[i].throw_to.1
                };
                monkeys[ind].items.push_back(item);
            }
        }
    }

    inspections.sort();
    let (m1, m2) = inspections.iter().rev().take(2).collect_tuple().unwrap();
    println!("Monkey business: {}", m1 * m2);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Plus(usize),
    Mult(usize),
    Square,
}

impl Operation {
    fn parse(input: Str) -> IResult<Str, Self> {
        let (input, (_, c, _, mult)) = tuple((
            tag("new = old "),
            alt((char('+'), char('*'))),
            char(' '),
            take_until("\n"),
        ))(input)?;

        match (c, mult) {
            ('+', "old") => Ok((input, Self::Mult(2))),
            ('*', "old") => Ok((input, Self::Square)),
            ('+', num) => {
                let (_, num) = map_res(digit1, str::parse)(num)?;
                Ok((input, Self::Plus(num)))
            }
            ('*', num) => {
                let (_, num) = map_res(digit1, str::parse)(num)?;
                Ok((input, Self::Mult(num)))
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Operation,
    pub test: usize,
    pub throw_to: (usize, usize),
}

fn parse_index(input: Str) -> IResult<Str, usize> {
    let (input, (_, index, _, _)) = tuple((
        tag("Monkey "),
        map_res(digit1, str::parse),
        tag(":"),
        line_ending,
    ))(input)?;
    Ok((input, index))
}

fn parse_items(input: Str) -> IResult<Str, VecDeque<usize>> {
    let (input, (_, vec, _)) = tuple((
        tag("  Starting items: "),
        map(
            separated_list0(tag(", "), map_res(digit1, str::parse)),
            VecDeque::from,
        ),
        line_ending,
    ))(input)?;
    Ok((input, vec))
}

fn parse_operation(input: Str) -> IResult<Str, Operation> {
    let (input, (_, operation, _)) =
        tuple((tag("  Operation: "), Operation::parse, line_ending))(input)?;
    Ok((input, operation))
}

fn parse_test(input: Str) -> IResult<Str, usize> {
    let (input, (_, number, _)) = tuple((
        tag("  Test: divisible by "),
        map_res(digit1, str::parse),
        line_ending,
    ))(input)?;
    Ok((input, number))
}

fn parse_throw_to(input: Str) -> IResult<Str, (usize, usize)> {
    let (input, (_, num_true, _, _, num_false, _)) = tuple((
        tag("    If true: throw to monkey "),
        map_res(digit1, str::parse),
        line_ending,
        tag("    If false: throw to monkey "),
        map_res(digit1, str::parse),
        many0(line_ending),
    ))(input)?;
    Ok((input, (num_true, num_false)))
}

fn parse_monkey(input: Str) -> IResult<Str, Monkey> {
    let (input, (_, items, operation, test, throw_to)) = tuple((
        parse_index,
        parse_items,
        parse_operation,
        parse_test,
        parse_throw_to,
    ))(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            throw_to,
        },
    ))
}
