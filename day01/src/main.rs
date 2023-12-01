use std::iter::once;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &'static str) {
    let sum: u32 = input
        .lines()
        .map(|l| l.chars().filter(|&c| char::is_numeric(c)))
        .map(Iterator::peekable)
        .map(|mut i| {
            i.peek().unwrap().to_digit(10).unwrap() * 10 + i.last().unwrap().to_digit(10).unwrap()
        })
        .sum();

    println!("Sum: {sum}");
}

fn part2(input: &'static str) {
    let parse = |tup| match tup {
        ('o', 'n', 'e', _, _) => 1,
        ('t', 'w', 'o', _, _) => 2,
        ('t', 'h', 'r', 'e', 'e') => 3,
        ('f', 'o', 'u', 'r', _) => 4,
        ('f', 'i', 'v', 'e', _) => 5,
        ('s', 'i', 'x', _, _) => 6,
        ('s', 'e', 'v', 'e', 'n') => 7,
        ('e', 'i', 'g', 'h', 't') => 8,
        ('n', 'i', 'n', 'e', _) => 9,
        _ => 0,
    };

    let sum: u32 = input
        .lines()
        .map(|l| {
            let mut i = l
                .chars()
                .chain(once('a'))
                .chain(once('a'))
                .chain(once('a'))
                .chain(once('a'))
                .tuple_windows()
                .map(|(a, b, c, d, e)| {
                    if a.is_numeric() {
                        a.to_digit(10).unwrap()
                    } else {
                        parse((a, b, c, d, e))
                    }
                })
                .filter(|&d| d != 0);
            let first = i.next().unwrap();
            let last = i.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();

    println!("Sum: {sum}");
}
