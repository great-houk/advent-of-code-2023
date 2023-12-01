use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{cmp::Ordering, fmt::Debug};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let (_, entries) = parse_part_1(input).unwrap();

    let mut sum = 0;
    for (i, (l, r)) in entries.iter().enumerate() {
        if should_swap(l, r) < 0 {
            sum += i + 1;
        }
    }

    dbg!(sum);
}

fn part2(input: Str) {
    let (_, mut entries) = parse_part_2(input).unwrap();
    let div2 = Entry::List(vec![Entry::List(vec![Entry::Number(2)])]);
    let div6 = Entry::List(vec![Entry::List(vec![Entry::Number(6)])]);
    entries.append(&mut vec![div2.clone(), div6.clone()]);
    entries.sort();

    let mut mult = 1;
    for (i, e) in entries.iter().enumerate() {
        if *e == div2 || *e == div6 {
            mult *= i + 1;
        }
    }

    dbg!(mult);
}

fn should_swap(l: &Entry, r: &Entry) -> isize {
    match (l, r) {
        (Entry::Number(l), Entry::Number(r)) => (l - r).signum(),
        (Entry::Number(_), Entry::List(rl)) => {
            if rl.len() == 0 {
                1
            } else {
                should_swap(&Entry::List(vec![l.clone()]), r)
            }
        }
        (Entry::List(ll), Entry::Number(_)) => {
            if ll.len() == 0 {
                -1
            } else {
                should_swap(&l, &Entry::List(vec![r.clone()]))
            }
        }
        (Entry::List(l), Entry::List(r)) => {
            if l.len() == 0 && r.len() == 0 {
                return 0;
            }
            if l.len() == 0 {
                return -1;
            }
            if r.len() == 0 {
                return 1;
            }
            for i in 0..r.len() {
                if i > l.len() - 1 {
                    return -1;
                }
                let dif = should_swap(&l[i], &r[i]);
                if dif != 0 {
                    return dif;
                }
            }
            if l.len() != r.len() {
                1
            } else {
                0
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Entry {
    Number(isize),
    List(Vec<Entry>),
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = should_swap(self, other);
        match s {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::List(list) => f.debug_list().entries(list).finish(),
        }
    }
}

fn parse_entry(input: Str) -> IResult<Str, Entry> {
    let (input, entry) = alt((
        map(digit1, |s: Str| Entry::Number(s.parse().unwrap())),
        parse_list,
    ))(input)?;
    Ok((input, entry))
}

fn parse_list(input: Str) -> IResult<Str, Entry> {
    let (i, v) = delimited(
        char('['),
        separated_list0(char(','), parse_entry),
        char(']'),
    )(input)?;
    Ok((i, Entry::List(v)))
}

fn parse_part_1(input: Str) -> IResult<Str, Vec<(Entry, Entry)>> {
    let (i, v) = separated_list1(
        tag("\n\n"),
        separated_pair(parse_list, char('\n'), parse_list),
    )(input)?;
    Ok((i, v))
}

fn parse_part_2(input: Str) -> IResult<Str, Vec<Entry>> {
    let (i, v) = separated_list1(many1(char('\n')), parse_list)(input)?;
    Ok((i, v))
}
