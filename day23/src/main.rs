use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut map = parse_input(input);
    let mut new_map = HashMap::new();

    for i in 0..10 {
        for (x, y) in map.iter().cloned() {
            let tl = map.contains(&(x - 1, y - 1));
            let tm = map.contains(&(x, y - 1));
            let tr = map.contains(&(x + 1, y - 1));
            let ml = map.contains(&(x - 1, y));
            let mr = map.contains(&(x + 1, y));
            let bl = map.contains(&(x - 1, y + 1));
            let bm = map.contains(&(x, y + 1));
            let br = map.contains(&(x + 1, y + 1));
            // All spaces are open, don't move
            if !(tl || tm || tr || ml || mr || bl || bm || br) {
                new_map.insert((x, y), (x, y));
                continue;
            }
            // Move dirs
            let check_cards: &[Box<dyn Fn(&mut HashMap<(i32, i32), (i32, i32)>) -> bool>] = &[
                // North
                Box::new(|new_map| {
                    if !(tl || tm || tr) {
                        if let Some(other) = new_map.remove(&(x, y - 1)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x, y - 1), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // South
                Box::new(|new_map| {
                    if !(bl || bm || br) {
                        if let Some(other) = new_map.remove(&(x, y + 1)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x, y + 1), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // West
                Box::new(|new_map| {
                    if !(tl || ml || bl) {
                        if let Some(other) = new_map.remove(&(x - 1, y)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x - 1, y), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // East
                Box::new(|new_map| {
                    if !(tr || mr || br) {
                        if let Some(other) = new_map.remove(&(x + 1, y)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x + 1, y), (x, y));
                        }
                        return true;
                    }
                    false
                }),
            ];
            // Go through moving
            let mut found = false;
            for j in 0..4 {
                if check_cards[(i + j) % 4](&mut new_map) {
                    found = true;
                    break;
                }
            }
            if !found {
                new_map.insert((x, y), (x, y));
            }
        }

        map.clear();
        map = new_map.iter().map(|((x, y), _)| (*x, *y)).collect();
        new_map.clear();
        // println!("\n\nRound {i}:");
        // print_map(&map);
    }

    let minx = *map.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *map.iter().map(|(x, _)| x).max().unwrap();
    let miny = *map.iter().map(|(_, y)| y).min().unwrap();
    let maxy = *map.iter().map(|(_, y)| y).max().unwrap();

    let area = (maxx - minx + 1) * (maxy - miny + 1);
    let num_tiles = map.len() as i32;
    dbg!(area - num_tiles);
}

fn part2(input: Str) {
    let mut map = parse_input(input);
    let mut new_map = HashMap::new();

    let mut i = 0;
    loop {
        let mut moved = false;

        for (x, y) in map.iter().cloned() {
            let tl = map.contains(&(x - 1, y - 1));
            let tm = map.contains(&(x, y - 1));
            let tr = map.contains(&(x + 1, y - 1));
            let ml = map.contains(&(x - 1, y));
            let mr = map.contains(&(x + 1, y));
            let bl = map.contains(&(x - 1, y + 1));
            let bm = map.contains(&(x, y + 1));
            let br = map.contains(&(x + 1, y + 1));
            // All spaces are open, don't move
            if !(tl || tm || tr || ml || mr || bl || bm || br) {
                new_map.insert((x, y), (x, y));
                continue;
            }
            // Move dirs
            let check_cards: &[Box<dyn Fn(&mut HashMap<(i32, i32), (i32, i32)>) -> bool>] = &[
                // North
                Box::new(|new_map| {
                    if !(tl || tm || tr) {
                        if let Some(other) = new_map.remove(&(x, y - 1)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x, y - 1), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // South
                Box::new(|new_map| {
                    if !(bl || bm || br) {
                        if let Some(other) = new_map.remove(&(x, y + 1)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x, y + 1), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // West
                Box::new(|new_map| {
                    if !(tl || ml || bl) {
                        if let Some(other) = new_map.remove(&(x - 1, y)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x - 1, y), (x, y));
                        }
                        return true;
                    }
                    false
                }),
                // East
                Box::new(|new_map| {
                    if !(tr || mr || br) {
                        if let Some(other) = new_map.remove(&(x + 1, y)) {
                            new_map.insert((x, y), (x, y));
                            new_map.insert(other, other);
                        } else {
                            new_map.insert((x + 1, y), (x, y));
                        }
                        return true;
                    }
                    false
                }),
            ];
            // Go through moving
            let mut found = false;
            for j in 0..4 {
                if check_cards[(i + j) % 4](&mut new_map) {
                    found = true;
                    moved = true;
                    break;
                }
            }
            if !found {
                new_map.insert((x, y), (x, y));
            }
        }

        map.clear();
        map = new_map.iter().map(|((x, y), _)| (*x, *y)).collect();
        new_map.clear();

        if !moved {
            break;
        }
        i += 1;
    }

    dbg!(i + 1);
}

fn print_map(map: &HashSet<(i32, i32)>) {
    let minx = *map.iter().map(|(x, _)| x).min().unwrap() - 2;
    let maxx = *map.iter().map(|(x, _)| x).max().unwrap() + 2;
    let miny = *map.iter().map(|(_, y)| y).min().unwrap() - 2;
    let maxy = *map.iter().map(|(_, y)| y).max().unwrap() + 2;

    for y in miny..=maxy {
        for x in minx..=maxx {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(input: Str) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for b in input.as_bytes() {
        match b {
            b'.' => x += 1,
            b'#' => {
                map.insert((x, y));
                x += 1;
            }
            b'\n' => {
                x = 0;
                y += 1;
            }
            _ => panic!("WOah"),
        }
    }
    map
}
