use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("sample.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    for line in input.lines() {
        let v = line
            .as_bytes()
            .windows(4)
            .enumerate()
            .filter_map(|(i, w)| {
                if (1..4).any(|j| w[j..].contains(&w[j - 1])) {
                    None
                } else {
                    Some(i + 4)
                }
            })
            .take(1)
            .collect_vec()[0];
        dbg!(v);
    }
}

fn part2(input: Str) {
    for line in input.lines() {
        let v = line
            .as_bytes()
            .windows(14)
            .enumerate()
            .filter_map(|(i, w)| {
                if (1..14).any(|j| w[j..].contains(&w[j - 1])) {
                    None
                } else {
                    Some(i + 14)
                }
            })
            .take(1)
            .collect_vec()[0];
        dbg!(v);
    }
}
