use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    combinator::{map_res, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    Err,
};
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Str = &'static str;
const ROW: isize = 2000000;
const MAX: isize = 4000000;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let map = parse_input(input).unwrap();
    let mut impossible = HashSet::new();

    for ((x, y), (bx, by)) in map {
        let dist = (x - bx).abs() + (y - by).abs();
        let xoff = dist - (ROW - y).abs();

        for x in x..=x + xoff {
            if by != ROW || bx != x {
                impossible.insert(x);
            }
        }
        for x in x - xoff..x {
            if by != ROW || bx != x {
                impossible.insert(x);
            }
        }
    }

    dbg!(impossible.len());
}

fn part2(input: Str) {
    let map = parse_input(input).unwrap();

    let mut ty = None;
    let mut tx = None;
    for row in 0..MAX {
        let mut ranges: Vec<Range> = vec![];

        for ((sx, sy), (bx, by)) in &map {
            let dist = (sx - bx).abs() + (sy - by).abs();
            let xoff = dist - (row - sy).abs();
            if xoff >= 0 {
                ranges.push((max(0, sx - xoff)..=min(MAX, sx + xoff)).into());
            }
        }

        let range = ranges
            .into_iter()
            .reduce(|mut acc, r| {
                acc.combine(r);
                acc
            })
            .unwrap();

        if range != (0..=MAX).into() {
            ty = Some(row);
            for x in 0..=MAX {
                if !range.contains(x) {
                    tx = Some(x);
                    break;
                }
            }
            break;
        }
    }
    let y = ty.unwrap();
    let x = tx.unwrap();
    dbg!((x, y));
    dbg!(x * 4000000 + y);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
    ranges: Vec<(isize, isize)>,
}

impl Range {
    fn combine(&mut self, mut other: Self) {
        'next: while let Some((l2, r2)) = other.ranges.pop() {
            for i in 0..self.ranges.len() {
                let (l1, r1) = self.ranges[i];
                if (r1 <= r2 && r1 >= l2 - 1)
                    || (l1 <= r2 && l1 >= l2 - 1)
                    || (r2 <= r1 && r2 >= l1 - 1)
                    || (l2 <= r1 && l2 >= l1 - 1)
                {
                    self.ranges.remove(i);
                    self.combine(Range {
                        ranges: vec![(min(l1, l2), max(r1, r2))],
                    });
                    continue 'next;
                }
            }
            self.ranges.push((l2, r2));
        }
    }

    fn contains(&self, other: isize) -> bool {
        for (l, r) in &self.ranges {
            if other >= *l && other <= *r {
                return true;
            }
        }
        false
    }
}

impl From<RangeInclusive<isize>> for Range {
    fn from(r: RangeInclusive<isize>) -> Self {
        Self {
            ranges: vec![(*r.start(), *r.end())],
        }
    }
}

fn parse_input(input: Str) -> Result<HashMap<(isize, isize), (isize, isize)>, Err<Error<Str>>> {
    let (_, v) = separated_list1(
        newline,
        tuple((
            separated_pair(
                preceded(
                    tag("Sensor at x="),
                    map_res(recognize(many1(one_of("-0123456789"))), str::parse::<isize>),
                ),
                tag(", y="),
                map_res(recognize(many1(one_of("-0123456789"))), str::parse::<isize>),
            ),
            separated_pair(
                preceded(
                    tag(": closest beacon is at x="),
                    map_res(recognize(many1(one_of("-0123456789"))), str::parse::<isize>),
                ),
                tag(", y="),
                map_res(recognize(many1(one_of("-0123456789"))), str::parse::<isize>),
            ),
        )),
    )(input)?;

    Ok(v.into_iter().collect())
}
