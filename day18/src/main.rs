use std::{
    io::{stdout, Write},
    process::Stdio,
};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, char, digit1, line_ending, newline},
    combinator::{all_consuming, map, map_res},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    Err,
};

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines = parse(input).unwrap();
    let mut flips = vec![];
    let mut coords = (0, 0);

    for (dir, dist, _) in lines {
        match dir {
            Dir::Up => {
                for _ in 0..dist {
                    flips.push((coords, dir));
                    coords.0 -= 1;
                }
                flips.push((coords, dir));
            }
            Dir::Down => {
                for _ in 0..dist {
                    flips.push((coords, dir));
                    coords.0 += 1;
                }
                flips.push((coords, dir));
            }
            Dir::Left => {
                coords.1 -= dist;
            }
            Dir::Right => {
                coords.1 += dist;
            }
        }
    }

    flips.sort();

    let mut coords = (0, 0);
    let max_x = *flips.iter().map(|((_, x), _)| x).max().unwrap();
    let mut inside = false;
    let mut ud = None;
    let mut count = 0;

    let mut print = |min, max, inside| {
        for _ in min..max {
            if inside {
                count += 1;
                print!("#")
            } else {
                print!(".")
            }
        }
        stdout().flush().unwrap();
    };

    for (flip, dir) in flips {
        // Finish previous line
        while coords.0 < flip.0 {
            print(coords.1, max_x + 1, inside);
            coords.0 += 1;
            coords.1 = 0;
            inside = false;
            ud = None;
            println!();
        }
        // Draw to next flip
        print(coords.1, flip.1, inside);
        coords = flip;
        // Determine next flip value
        if let Some(d) = ud {
            inside = d == dir;
            ud = None;
        } else {
            inside = true;
            ud = Some(dir);
        }
    }
    println!();

    println!("Count: {count}");
}
fn part2(input: &str) {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> Result<Vec<(Dir, usize, u32)>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        line_ending,
        tuple((
            map_res(anychar, |c| {
                Ok(match c {
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => return Err(format!("Unexpected char {c}")),
                })
            }),
            preceded(char(' '), map_res(digit1, str::parse)),
            terminated(
                preceded(
                    tag(" (#"),
                    map_res(take(6usize), |n| u32::from_str_radix(n, 16)),
                ),
                char(')'),
            ),
        )),
    ))(input)
    .map(|r| r.1)
}
