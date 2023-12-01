use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::char,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    Err,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let (map, moves) = parse_input(input).unwrap();
    let minx = map.iter().map(|((x, _), _)| x).min().unwrap();

    let mut path = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for (dist, dir) in moves.iter().cloned() {
        for _ in 0..dist {
            path.insert((x, y));

            let (dx, dy) = match dir {
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Up => (0, -1),
            };

            match map.get(&(x + dx, y + dy)) {
                Some(Square::Space) => (x, y) = (x + dx, y + dy),
                None => {
                    let (mut nx, mut ny) = (x, y);
                    while let Some(_) = map.get(&(nx, ny)) {
                        nx -= dx;
                        ny -= dy;
                    }
                    nx += dx;
                    ny += dy;
                    if let Some(Square::Space) = map.get(&(nx, ny)) {
                        (x, y) = (nx, ny);
                    }
                }
                Some(Square::Wall) => (),
            }
        }
    }

    print_input(&map, &path);
    let last = moves.last().unwrap().1.value();
    dbg!(minx, x, y, last);
    dbg!(1000 * (y + 1) + 4 * (x - minx + 1) + last);
}

fn part2(input: Str) {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Square {
    Wall,
    Space,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    pub fn value(&self) -> isize {
        *self as u8 as isize
    }
}

fn print_input(map: &HashMap<(isize, isize), Square>, path: &HashSet<(isize, isize)>) {
    let xmin = *map.iter().map(|((x, _), _)| x).min().unwrap();
    let xmax = *map.iter().map(|((x, _), _)| x).max().unwrap();
    let ymax = *map.iter().map(|((_, y), _)| y).max().unwrap();

    for y in 0..=ymax {
        for x in xmin..=xmax {
            print!(
                "{}",
                if let Some(v) = map.get(&(x, y)) {
                    if path.contains(&(x, y)) {
                        '@'
                    } else {
                        match v {
                            Square::Space => '.',
                            Square::Wall => '#',
                        }
                    }
                } else {
                    ' '
                }
            );
        }
        println!();
    }
}

fn parse_input(
    input: Str,
) -> Result<(HashMap<(isize, isize), Square>, Vec<(u32, Direction)>), Err<Error<Str>>> {
    let (m, moves) = input.split_once("\n\n").unwrap();
    let xoff = m.as_bytes().iter().position(|b| *b != b' ').unwrap() as isize;
    let mut maze = HashMap::new();

    let mut y = 0;
    let mut x = -xoff;
    for b in m.as_bytes() {
        match b {
            b' ' => x += 1,
            b'.' => {
                maze.insert((x, y), Square::Space);
                x += 1;
            }
            b'#' => {
                maze.insert((x, y), Square::Wall);
                x += 1;
            }
            b'\n' => {
                x = -xoff;
                y += 1;
            }
            _ => panic!("Not supposed to hit here: {b}"),
        }
    }

    let mut dir = RefCell::new(Direction::Right);
    let (_, moves): (_, Vec<(u32, Direction)>) = all_consuming(separated_list1(
        map(
            alt((
                map(char('L'), |_| match dir.borrow().clone() {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                }),
                map(char('R'), |_| match dir.borrow().clone() {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }),
            )),
            |d| *dir.borrow_mut() = d,
        ),
        map(map_res(digit1, str::parse), |n| (n, dir.borrow().clone())),
    ))(moves)?;

    Ok((maze, moves))
}
