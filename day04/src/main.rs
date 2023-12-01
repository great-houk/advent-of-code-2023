use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let count: i32 = input
        .lines()
        .map(|l| {
            let t = l
                .split(',')
                .map(|p| {
                    p.split('-')
                        .map(str::parse::<i32>)
                        .map(Result::unwrap)
                        .next_tuple::<(_, _)>()
                        .unwrap()
                })
                .next_tuple::<(_, _)>()
                .unwrap();
            if ((t.0 .1 - t.1 .1).signum() + (t.0 .0 - t.1 .0).signum()).abs() < 2 {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Count is {count}");
}

fn part2(input: Str) {
    let count: i32 = input
        .lines()
        .map(|l| {
            let t = l
                .split(',')
                .map(|p| {
                    p.split('-')
                        .map(str::parse::<i32>)
                        .map(Result::unwrap)
                        .next_tuple::<(_, _)>()
                        .unwrap()
                })
                .next_tuple::<(_, _)>()
                .unwrap();
            let first = ((1u128 << (t.0 .1 + 1)) - 1) ^ ((1u128 << t.0 .0) - 1);
            let second = ((1u128 << (t.1 .1 + 1)) - 1) ^ ((1u128 << t.1 .0) - 1);
            if first ^ second != first + second {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Count is {count}");
}
