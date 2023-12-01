use std::collections::VecDeque;

use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let mut lines = input.lines().peekable();
    let len = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(len);
    for _ in 0..len {
        stacks.push(VecDeque::new());
    }
    // Get stack initial state
    for line in &mut lines {
        if line == "" {
            break;
        }

        let chars = line.as_bytes();
        for i in 0..len {
            let ind = 1 + i * 4;
            if chars[ind] != b' ' {
                stacks[i].push_front(chars[ind] as char);
            }
        }
    }
    // Remove number from top of stack
    for v in &mut stacks {
        v.pop_front();
    }
    // Parse following lines
    for line in &mut lines {
        let (amount, from, to) = {
            let words: Vec<_> = line
                .split_ascii_whitespace()
                .tuples()
                .map(|(_, i)| i.parse::<usize>().unwrap())
                .collect();
            (words[0], words[1] - 1, words[2] - 1)
        };

        let l = stacks[from].len();
        let mut temp = stacks[from].drain(l - amount..).rev().collect();
        stacks[to].append(&mut temp);
    }
    // Get end
    let mut output = String::new();
    for mut stack in stacks {
        output += &stack.pop_back().unwrap().to_string();
    }
    println!("Top elements are {output}");
}

fn part2(input: Str) {
    let mut lines = input.lines().peekable();
    let len = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(len);
    for _ in 0..len {
        stacks.push(VecDeque::new());
    }
    // Get stack initial state
    for line in &mut lines {
        if line == "" {
            break;
        }

        let chars = line.as_bytes();
        for i in 0..len {
            let ind = 1 + i * 4;
            if chars[ind] != b' ' {
                stacks[i].push_front(chars[ind] as char);
            }
        }
    }
    // Remove number from top of stack
    for v in &mut stacks {
        v.pop_front();
    }
    // Parse following lines
    for line in &mut lines {
        let (amount, from, to) = {
            let words: Vec<_> = line
                .split_ascii_whitespace()
                .tuples()
                .map(|(_, i)| i.parse::<usize>().unwrap())
                .collect();
            (words[0], words[1] - 1, words[2] - 1)
        };

        let l = stacks[from].len();
        let mut temp = stacks[from].drain(l - amount..).collect();
        stacks[to].append(&mut temp);
    }
    // Get end
    let mut output = String::new();
    for mut stack in stacks {
        output += &stack.pop_back().unwrap().to_string();
    }
    println!("Top elements are {output}");
}
