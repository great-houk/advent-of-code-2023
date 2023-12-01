use itertools::Itertools;
use std::collections::HashMap;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut map: HashMap<String, u64> = HashMap::new();
    map.insert("/".to_string(), 0);
    let mut cur = vec!["/".to_string()];
    for line in input.lines() {
        let words = line.split_whitespace().collect_vec();
        if words[0] == "$" && words[1] == "cd" {
            match words[2] {
                ".." => {
                    cur.pop();
                }
                "/" => cur = vec!["/".to_string()],
                _ => {
                    let dir = cur[cur.len() - 1].clone() + "/" + &words[2].to_string();
                    if !map.contains_key(&dir) {
                        map.insert(dir.clone(), 0);
                    }
                    cur.push(dir);
                }
            }
        } else if let Ok(size) = words[0].parse::<u64>() {
            for dir in &cur {
                *map.get_mut(dir).unwrap() += size;
            }
        }
    }
    let sum: u64 = map.values().filter(|v| **v < 100_000).sum();
    println!("Sum {sum:?}");
}

fn part2(input: Str) {
    let mut map: HashMap<String, u64> = HashMap::new();
    map.insert("/".to_string(), 0);
    let mut cur = vec!["/".to_string()];
    for line in input.lines() {
        let words = line.split_whitespace().collect_vec();
        if words[0] == "$" && words[1] == "cd" {
            match words[2] {
                ".." => {
                    cur.pop();
                }
                "/" => cur = vec!["/".to_string()],
                _ => {
                    let dir = cur[cur.len() - 1].clone() + "/" + &words[2].to_string();
                    if !map.contains_key(&dir) {
                        map.insert(dir.clone(), 0);
                    }
                    cur.push(dir);
                }
            }
        } else if let Ok(size) = words[0].parse::<u64>() {
            for dir in &cur {
                *map.get_mut(dir).unwrap() += size;
            }
        }
    }
    let needed = 30_000_000 - (70_000_000 - map.get("/").unwrap());
    let sum: u64 = *map.values().filter(|v| **v > needed).min().unwrap();
    println!("Min is {sum:?}");
}
