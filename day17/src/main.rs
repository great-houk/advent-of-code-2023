use std::{cmp::max, collections::HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::char,
    character::complete::line_ending,
    combinator::{all_consuming, value},
    error::Error,
    multi::many1,
    sequence::terminated,
    Err,
};

type Str = &'static str;

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let jets = parse_input(input).unwrap().into_iter().cycle();
    let mut piece = Piece::Line;
    let mut board = HashSet::new();
    let mut tallest = 0;
    let mut pos = (0, tallest + 4);
    let mut count = 0;

    for jet in jets {
        // Jet Move
        if piece.can_move(jet, 0, &pos, &board) {
            pos.0 += jet;
        }
        // Down Move
        if piece.can_move(0, -1, &pos, &board) {
            pos.1 -= 1;
        } else {
            tallest = max(tallest, piece.place(&pos, &mut board));
            count += 1;
            piece.next();
            pos = (0, tallest + 4);
            if count > 2021 {
                break;
            }
        }
    }
    // print_board(&board, tallest + 4);
    dbg!(tallest);
}

fn part2(input: Str) {
    let jets = parse_input(input).unwrap().into_iter().cycle();
    let mut piece = Piece::Line;
    let mut board = HashSet::new();
    let mut tallest = 0;
    let mut pos = (0, tallest + 4);
    let mut count = 0i64;

    for jet in jets {
        // Jet Move
        if piece.can_move(jet, 0, &pos, &board) {
            pos.0 += jet;
        }
        // Down Move
        if piece.can_move(0, -1, &pos, &board) {
            pos.1 -= 1;
        } else {
            tallest = max(tallest, piece.place(&pos, &mut board));
            count += 1;
            piece.next();
            pos = (0, tallest + 4);
            if count > 1_000_000_000_000 - 1 {
                break;
            }
        }
    }
    // print_board(&board, tallest + 4);
    dbg!(tallest);
}

fn print_board(board: &HashSet<(i64, i64)>, tallest: i64) {
    println!("+-------+");
    for y in (0..=tallest).rev() {
        if y != 0 {
            print!("|");
            for x in -3..=3 {
                if board.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        } else {
            println!("+-------+");
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Piece {
    Line,
    Cross,
    L,
    I,
    Square,
}

impl Piece {
    fn next(&mut self) {
        *self = match self {
            Self::Line => Self::Cross,
            Self::Cross => Self::L,
            Self::L => Self::I,
            Self::I => Self::Square,
            Self::Square => Self::Line,
        };
    }

    fn can_move(&self, x: i64, y: i64, pos: &(i64, i64), board: &HashSet<(i64, i64)>) -> bool {
        let (x, y) = (pos.0 + x, pos.1 + y);
        // Check floor
        if y < 1 {
            return false;
        }
        // Check Walls
        if match self {
            Piece::Line => x > 1 || x < -2,
            Piece::Cross => x > 2 || x < -2,
            Piece::L => x > 2 || x < -2,
            Piece::I => x > 4 || x < -2,
            Piece::Square => x > 3 || x < -2,
        } {
            return false;
        }
        // Check other pieces
        for (ox, oy) in self.tiles() {
            if board.contains(&(x + ox, y + oy)) {
                return false;
            }
        }
        // Nothing collides
        true
    }

    fn place(&self, pos: &(i64, i64), board: &mut HashSet<(i64, i64)>) -> i64 {
        let mut tallest = 0;
        for tile in self.tiles() {
            board.insert((pos.0 + tile.0, pos.1 + tile.1));
            tallest = max(pos.1 + tile.1, tallest);
        }
        tallest
    }

    fn tiles(&self) -> &[(i64, i64)] {
        match self {
            Self::Line => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
            Self::Cross => &[(0, 0), (-1, 1), (0, 1), (1, 1), (0, 2)],
            Self::L => &[(-1, 0), (0, 0), (1, 0), (1, 1), (1, 2)],
            Self::I => &[(-1, 0), (-1, 1), (-1, 2), (-1, 3)],
            Self::Square => &[(-1, 0), (0, 0), (-1, 1), (0, 1)],
        }
    }
}

fn parse_input(input: Str) -> Result<Vec<i64>, Err<Error<Str>>> {
    let (_, v) = all_consuming(many1(alt((value(-1, char('<')), value(1, char('>'))))))(input)?;
    Ok(v)
}
