fn main() {
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut input: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let y_max = input.len() as isize;
    let x_max = input[0].len() as isize;

    let mut sum = 0;
    for y in 0..y_max {
        for x in 0..x_max {
            if input[y as usize][x as usize] != '.' && !input[y as usize][x as usize].is_numeric() {
                for yoff in -1..=1 {
                    for xoff in -1..=1 {
                        sum += find_num(x + xoff, y + yoff, &mut input)
                            .parse::<u32>()
                            .unwrap_or(0);
                    }
                }
            }
        }
    }

    println!("Sum: {sum}");
}

fn part2(input: &str) {
    let mut input: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let y_max = input.len() as isize;
    let x_max = input[0].len() as isize;

    let mut sum = 0;
    for y in 0..y_max {
        for x in 0..x_max {
            if input[y as usize][x as usize] == '*' {
                let mut count = 0;
                let mut mult = 1;

                for yoff in -1..=1 {
                    for xoff in -1..=1 {
                        if let Ok(num) = find_num(x + xoff, y + yoff, &mut input).parse::<u32>() {
                            mult *= num;
                            count += 1;
                        }
                    }
                }

                if count == 2 {
                    sum += mult;
                }
            }
        }
    }

    println!("Gear Ratio Sum: {sum}");
}

fn find_num(x: isize, y: isize, input: &mut Vec<Vec<char>>) -> String {
    if x < 0
        || y < 0
        || y >= input.len() as isize
        || x >= input[y as usize].len() as isize
        || !input[y as usize][x as usize].is_numeric()
    {
        return "".to_string();
    }

    let string = input[y as usize][x as usize].to_string();
    input[y as usize][x as usize] = '.';
    find_num(x - 1, y, input) + &string + &find_num(x + 1, y, input)
}
