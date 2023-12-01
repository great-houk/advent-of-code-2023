use itertools::Itertools;

type Str = &'static str;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: Str) {
    let v = parse_input(input);

    let mut sum: isize = v.iter().sum();
    dbg!(sum);

    let mut carry = 0;
    let mut snafu = String::new();

    while sum != 0 {
        let mut add = sum % 5;

        if carry > 0 {
            add += carry;
            carry -= 1;
        }
        if add > 2 {
            carry += 1;
            add -= 5;
        }

        match add {
            -2 => snafu = "=".to_owned() + &snafu,
            -1 => snafu = "-".to_owned() + &snafu,
            0 => snafu = "0".to_owned() + &snafu,
            1 => snafu = "1".to_owned() + &snafu,
            2 => snafu = "2".to_owned() + &snafu,
            _ => panic!("WOAh"),
        }

        sum /= 5;
    }

    dbg!(snafu);
}

fn part2(input: Str) {}

fn parse_input(input: Str) -> Vec<isize> {
    let mut v = Vec::new();

    for line in input.lines() {
        let mut num = 0;
        for b in line.as_bytes() {
            num *= 5;
            match b {
                b'=' => num -= 2,
                b'-' => num -= 1,
                b'0' => (),
                b'1' => num += 1,
                b'2' => num += 2,
                _ => panic!("WOah"),
            }
        }
        v.push(num);
    }

    v
}
