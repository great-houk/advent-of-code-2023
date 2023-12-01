use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let instructions = input
        .lines()
        .map(|l| match l {
            "noop" => Instruction::Noop(true),
            _ => {
                let (_, v) = l.split_ascii_whitespace().collect_tuple().unwrap();
                Instruction::Addx(true, v.parse().unwrap(), false)
            }
        })
        .flatten()
        .enumerate();

    let mut x = 1;
    let mut sum = 0;

    for (i, v) in instructions {
        // 1 based indexing
        let i = i + 1;
        if (i as isize - 20) % 40 == 0 {
            sum += dbg!(i as isize * x as isize);
        }
        x += v;
    }

    dbg!(sum);
}

fn part2(input: Str) {
    let instructions = input
        .lines()
        .map(|l| match l {
            "noop" => Instruction::Noop(true),
            _ => {
                let (_, v) = l.split_ascii_whitespace().collect_tuple().unwrap();
                Instruction::Addx(true, v.parse().unwrap(), false)
            }
        })
        .flatten()
        .enumerate();

    let mut sprite = 1;
    let mut output = String::new();
    for (cycle, v) in instructions {
        let cycle = cycle as i32;
        if cycle % 40 == 0 {
            output += "\n";
        }
        if (sprite - (cycle % 40)).abs() < 2 {
            output += "#";
        } else {
            output += ".";
        }
        sprite += v;
    }

    println!("{output}");
}

enum Instruction {
    Noop(bool),
    Addx(bool, i32, bool),
}

impl Iterator for Instruction {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Noop(false) | Self::Addx(false, _, _) => None,
            Self::Noop(b) => {
                *b = !*b;
                Some(0)
            }
            Self::Addx(a, v, b) => {
                *b = !*b;
                if *b {
                    Some(0)
                } else {
                    *a = !*a;
                    Some(*v)
                }
            }
        }
    }
}
