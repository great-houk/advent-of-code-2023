use std::ops::AddAssign;

use colored::*;
use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part1quick(input);
    part2(input);
}

fn part1(input: Str) {
    let mut trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| (c.to_digit(10).unwrap() as i32, false))
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;
    for i in 0..trees.len() {
        'next: for j in 0..trees[i].len() {
            // Left
            let mut min = -1;
            for x in 0..j {
                if trees[i][x].0 >= min {
                    min = trees[i][x].0;
                }
            }
            if trees[i][j].0 > min {
                sum += 1;
                trees[i][j].1 |= true;
                continue 'next;
            }
            // Right
            min = -1;
            for x in (j + 1..trees[i].len()).rev() {
                if trees[i][x].0 >= min {
                    min = trees[i][x].0;
                }
            }
            if trees[i][j].0 > min {
                sum += 1;
                trees[i][j].1 |= true;
                continue 'next;
            }
            // Up
            min = -1;
            for y in 0..i {
                if trees[y][j].0 >= min {
                    min = trees[y][j].0;
                }
            }
            if trees[i][j].0 > min {
                sum += 1;
                trees[i][j].1 |= true;
                continue 'next;
            }
            // Down
            min = -1;
            for y in (i + 1..trees.len()).rev() {
                if trees[y][j].0 >= min {
                    min = trees[y][j].0;
                }
            }
            if trees[i][j].0 > min {
                sum += 1;
                trees[i][j].1 |= true;
                continue 'next;
            }
        }
    }

    for l in &trees {
        for t in l {
            if t.1 {
                print!("{} ", t.0.to_string().green());
            } else {
                print!("{} ", t.0.to_string().red());
            }
        }
        println!();
    }

    dbg!(sum);
}

fn part2(input: Str) {
    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;
    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            // Left
            let mut left = 0;
            for x in (0..j).rev() {
                if trees[i][x] < trees[i][j] {
                    left += 1;
                } else {
                    left += 1;
                    break;
                }
            }
            // Right
            let mut right = 0;
            for x in j + 1..trees[i].len() {
                if trees[i][x] < trees[i][j] {
                    right += 1;
                } else {
                    right += 1;
                    break;
                }
            }
            // Up
            let mut up = 0;
            for y in (0..i).rev() {
                if trees[y][j] < trees[i][j] {
                    up += 1;
                } else {
                    up += 1;
                    break;
                }
            }
            // Down
            let mut down = 0;
            for y in i + 1..trees.len() {
                if trees[y][j] < trees[i][j] {
                    down += 1;
                } else {
                    down += 1;
                    break;
                }
            }
            // Sum
            if up * down * left * right > sum {
                sum = up * down * left * right;
            }
        }
    }

    dbg!(sum);
}
