use std::collections::{HashMap, VecDeque};

use nom::{
    character::complete::{anychar, newline},
    combinator::{all_consuming, map_res},
    error::VerboseError,
    multi::{many1, separated_list1},
    Err,
};

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Dir {
    Start,
    Left,
    Right,
    Down,
    Up,
}

impl Dir {
    pub fn add(
        self,
        dist: usize,
        (row, col): &(usize, usize),
        grid: &Vec<Vec<u32>>,
    ) -> Option<(usize, usize)> {
        Some(match self {
            Dir::Left if *col >= dist => (*row, col - dist),
            Dir::Right if *col < grid[0].len() - dist => (*row, col + dist),
            Dir::Down if *row >= dist => (row - dist, *col),
            Dir::Up if *row < grid.len() - dist => (row + dist, *col),
            _ => return None,
        })
    }
}

fn part1(input: &str) {
    let grid = parse(input).unwrap();
    let mut history = HashMap::new();
    let mut places = VecDeque::new();
    let mut min = u32::MAX;
    places.push_back(((0, 0, Dir::Start), 0));

    while let Some(((row, col, dir), lost)) = places.pop_front() {
        if row == grid.len() - 1 && col == grid[0].len() - 1 {
            if lost < min {
                min = lost;
            }
            continue;
        }

        if let Some(l) = history.get_mut(&(row, col, dir)) {
            if lost >= *l {
                continue;
            } else {
                *l = lost;
            }
        } else {
            history.insert((row, col, dir), lost);
        }

        let dirs = match dir {
            Dir::Start => [Dir::Right, Dir::Up],
            Dir::Right | Dir::Left => [Dir::Up, Dir::Down],
            Dir::Up | Dir::Down => [Dir::Right, Dir::Left],
        };

        for dir in dirs {
            let mut cost = lost;
            for dist in 1..=3 {
                if let Some((row, col)) = dir.add(dist, &(row, col), &grid) {
                    cost += grid[row][col];
                    places.push_back(((row, col, dir), cost));
                }
            }
        }
    }

    println!("Min: {min}");
}

fn part2(input: &str) {
    let grid = parse(input).unwrap();
    let mut history = HashMap::new();
    let mut places = VecDeque::new();
    let mut min = u32::MAX;
    places.push_back(((0, 0, Dir::Start), 0));

    while let Some(((row, col, dir), lost)) = places.pop_front() {
        if row == grid.len() - 1 && col == grid[0].len() - 1 {
            if lost < min {
                min = lost;
            }
            continue;
        }

        if let Some(l) = history.get_mut(&(row, col, dir)) {
            if lost >= *l {
                continue;
            } else {
                *l = lost;
            }
        } else {
            history.insert((row, col, dir), lost);
        }

        let dirs = match dir {
            Dir::Start => [Dir::Right, Dir::Up],
            Dir::Right | Dir::Left => [Dir::Up, Dir::Down],
            Dir::Up | Dir::Down => [Dir::Right, Dir::Left],
        };

        for dir in dirs {
            let mut cost = lost;
            for dist in 1..=10 {
                if let Some((row, col)) = dir.add(dist, &(row, col), &grid) {
                    cost += grid[row][col];
                    if dist >= 4 {
                        places.push_back(((row, col, dir), cost));
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("Min: {min}");
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        many1(map_res(anychar, |c| c.to_digit(10).ok_or(()))),
    ))(input)
    .map(|r| r.1)
}
