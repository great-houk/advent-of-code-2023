use std::collections::HashSet;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn step(
    row: usize,
    col: usize,
    dir: Dir,
    lights: &mut Vec<((usize, usize), Dir)>,
    grid: &Vec<Vec<char>>,
) {
    let left = col > 0;
    let right = col < grid[0].len() - 1;
    let up = row < grid.len() - 1;
    let down = row > 0;

    match (grid[row][col], dir) {
        ('.' | '-', Dir::Right) if right => lights.push(((row, col + 1), Dir::Right)),
        ('\\', Dir::Right) if up => lights.push(((row + 1, col), Dir::Up)),
        ('/', Dir::Right) if down => lights.push(((row - 1, col), Dir::Down)),
        ('.' | '-', Dir::Left) if left => lights.push(((row, col - 1), Dir::Left)),
        ('\\', Dir::Left) if down => lights.push(((row - 1, col), Dir::Down)),
        ('/', Dir::Left) if up => lights.push(((row + 1, col), Dir::Up)),
        ('|', Dir::Right | Dir::Left) => {
            if down {
                lights.push(((row - 1, col), Dir::Down));
            }
            if up {
                lights.push(((row + 1, col), Dir::Up));
            }
        }

        ('.' | '|', Dir::Up) if up => lights.push(((row + 1, col), Dir::Up)),
        ('\\', Dir::Up) if right => lights.push(((row, col + 1), Dir::Right)),
        ('/', Dir::Up) if left => lights.push(((row, col - 1), Dir::Left)),
        ('.' | '|', Dir::Down) if down => lights.push(((row - 1, col), Dir::Down)),
        ('\\', Dir::Down) if left => lights.push(((row, col - 1), Dir::Left)),
        ('/', Dir::Down) if right => lights.push(((row, col + 1), Dir::Right)),
        ('-', Dir::Down | Dir::Up) => {
            if right {
                lights.push(((row, col + 1), Dir::Right));
            }
            if left {
                lights.push(((row, col - 1), Dir::Left));
            }
        }
        _ => (),
    }
}

fn part1(input: &str) {
    let grid = parse(input).unwrap();
    let mut lights = vec![((0, 0), Dir::Right)];
    let mut energized = vec![vec![false; grid[0].len()]; grid.len()];
    let mut history = HashSet::new();

    while let Some(((row, col), dir)) = lights.pop() {
        if !history.insert((row, col, dir)) {
            continue;
        }
        energized[row][col] = true;

        step(row, col, dir, &mut lights, &grid);
    }

    // for row in &energized {
    //     for c in row {
    //         if *c {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!();

    let count = energized
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| *c)
        .count();
    println!("Count: {count}");
}

fn part2(input: &str) {
    let grid = parse(input).unwrap();
    let mut max = 0;
    let mut starts = vec![];
    for col in 0..grid[0].len() {
        starts.push(((0, col), Dir::Up));
        starts.push(((grid.len() - 1, col), Dir::Down));
    }
    for row in 0..grid.len() {
        starts.push(((row, 0), Dir::Right));
        starts.push(((row, grid[0].len() - 1), Dir::Left));
    }
    // let mut best = vec![vec![false; grid[0].len()]; grid.len()];

    for start in starts {
        let mut lights = vec![start];
        let mut energized = vec![vec![false; grid[0].len()]; grid.len()];
        let mut history = HashSet::new();

        while let Some(((row, col), dir)) = lights.pop() {
            if !history.insert((row, col, dir)) {
                continue;
            }
            energized[row][col] = true;

            step(row, col, dir, &mut lights, &grid);
        }

        let count = energized
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| **c)
            .count();
        if count > max {
            max = count;
            // best = energized;
        }
    }

    // for row in &best {
    //     for c in row {
    //         if *c {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!();

    println!("Max: {max}");
}

fn parse(input: &str) -> Result<Vec<Vec<char>>, Err<VerboseError<&str>>> {
    all_consuming(separated_list1(
        newline,
        many1(map_res(
            anychar,
            |c| if c == '\n' { Err(()) } else { Ok(c) },
        )),
    ))(input)
    .map(|r| r.1)
}
