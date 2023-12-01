// use colored::Colorize;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{anychar, newline},
    combinator::{all_consuming, eof, map, map_res, opt},
    error::Error,
    multi::{many1, many_till},
    sequence::{terminated, tuple},
    Err,
};
use std::collections::{HashMap, VecDeque};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let map = parse_map(input);
    let mut visited = HashMap::new();
    let mut moves = VecDeque::new();
    let valid = |x: usize, y: usize, val| {
        if y > map.len() - 1 || x > map[y].len() - 1 {
            return None;
        }
        if val + 1 == map[y][x] || val >= map[y][x] {
            return Some(map[y][x]);
        }
        None
    };
    // Find start
    'found: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(val) = valid(x, y, -1) {
                moves.push_back((x, y, val, 0));
                visited.insert((x, y), 0);
                break 'found;
            }
        }
    }
    // Search
    let mut total_count = 0;
    while let Some((x, y, val, count)) = moves.pop_front() {
        // Test end
        if map[y][x] == 'z' as isize - 'a' as isize + 2 {
            total_count = count;
            break;
        }
        // Find moves
        let mut highest = -1;
        let mut test = |x: usize, y: usize| {
            if let Some(higher) = valid(x, y, val) {
                if higher >= highest {
                    highest = higher;
                    if let Some(&v) = visited.get(&(x, y)) {
                        if count + 1 < v {
                            visited.insert((x, y), count + 1);
                            moves.push_back((x, y, highest, count + 1));
                        }
                    } else {
                        visited.insert((x, y), count + 1);
                        moves.push_back((x, y, highest, count + 1));
                    }
                }
            }
        };
        if x > 0 {
            test(x - 1, y);
        }
        if y > 0 {
            test(x, y - 1);
        }
        test(x + 1, y);
        test(x, y + 1);
        let _a = 5;
    }

    dbg!(total_count);
}

fn part2(input: Str) {
    let map = parse_map_nom(input).unwrap();
    let mut visited = HashMap::new();
    let mut moves = VecDeque::new();
    let valid = |x: usize, y: usize, val| {
        if y > map.len() - 1 || x > map[y].len() - 1 {
            return None;
        }
        if val + 1 == map[y][x] || val >= map[y][x] {
            return Some(map[y][x]);
        }
        None
    };
    // Find start
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(val) = valid(x, y, 0) {
                moves.push_back((x, y, val, 0));
                visited.insert((x, y), 0);
            }
        }
    }
    // Search
    let mut min_count = -1;
    while let Some((x, y, val, count)) = moves.pop_front() {
        // Test end
        if map[y][x] == 'z' as isize - 'a' as isize + 2 {
            if min_count == -1 || count < min_count {
                min_count = count;
            }
            continue;
        }
        // Find moves
        let mut highest = -1;
        let mut test = |x: usize, y: usize| {
            if let Some(higher) = valid(x, y, val) {
                if higher >= highest {
                    highest = higher;
                    if let Some(&v) = visited.get(&(x, y)) {
                        if count + 1 < v {
                            visited.insert((x, y), count + 1);
                            moves.push_back((x, y, highest, count + 1));
                        }
                    } else {
                        visited.insert((x, y), count + 1);
                        moves.push_back((x, y, highest, count + 1));
                    }
                }
            }
        };
        if x > 0 {
            test(x - 1, y);
        }
        if y > 0 {
            test(x, y - 1);
        }
        test(x + 1, y);
        test(x, y + 1);
        let _a = 5;
    }

    // for y in 0..map.len() {
    //     for x in 0..map[y].len() {
    //         let c = (map[y][x] as u8 + b'a' - 1) as char;
    //         if let Some(_) = visited.get(&(x, y)) {
    //             print!("{}", c.to_string().green());
    //         } else {
    //             print!("{}", c.to_string().red());
    //         }
    //     }
    //     println!();
    // }

    dbg!(min_count);
}

fn parse_map(input: Str) -> Vec<Vec<isize>> {
    let mut map = vec![Vec::new()];
    for c in input.chars() {
        if let Some(c) = match c {
            'S' => Some(0),
            'E' => Some('z' as isize - 'a' as isize + 2),
            '\n' => {
                map.push(Vec::new());
                None
            }
            c => Some(c as isize - 'a' as isize + 1),
        } {
            map.last_mut().unwrap().push(c);
        }
    }
    map
}

fn parse_map_nom(input: Str) -> Result<Vec<Vec<isize>>, Err<Error<Str>>> {
    let (_, (map, _)) = all_consuming(many_till(
        map(
            tuple((
                many1(map_res(anychar, |c| match c {
                    'S' => Ok(0),
                    'E' => Ok('z' as isize - 'a' as isize + 2),
                    'a'..='z' => Ok(c as isize - 'a' as isize + 1),
                    _ => Err(input),
                })),
                opt(newline),
            )),
            |(line, _)| line,
        ),
        eof,
    ))(input)?;
    Ok(map)
}
