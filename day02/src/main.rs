use std::arch::asm;

fn main() {
    let input = include_str!("input.txt");

    assembly1(input);
    part1(input);
    assembly2(input);
    part2(input);
}

fn assembly1(input: &'static str) {
    let sum: u64;
    const WIN_TABLE: [u8; 3] = [3, 0, 6];

    unsafe {
        asm! {
            "mov {sum}, 0",
            // Setup length
            "shr {len}, 2",
            "2:",
            // Store 4 chars in {str}
            "mov {arr:e}, [{str} + {len} * 4 - 4]",
            // Mask space and newline
            "sub {arr}, 0x00580041",
            "mov {tmp}, 0x0000000000FF00FF",
            "and {arr}, {tmp}",
            // Calculate offset of comp move
            // and player move. [-2, -1, 0, 1, 2]
            "mov {tmp}, 0",
            "mov {tmp:l}, {arr:l}",
            "shr {arr}, 16",
            "sub {tmp}, {arr}",
            // Make everything positive
            // -2 => 1, -1 => 2
            "jns 3f",
            "add {tmp}, 3",
            "3:",
            // Use win table to add bonus
            "mov {tmp:l}, [{win} + {tmp}]",
            "and {tmp}, 0xFF",
            "add {sum}, {tmp}",
            // Add move
            "mov {tmp:l}, {arr:l}",
            "and {tmp}, 0xFF",
            "add {sum}, {tmp}",
            "inc {sum}",
            // Loop
            "dec {len}",
            "jnz 2b",
            // Holds the pointer to the string
            str = inout(reg) input as *const str as *const u8 => _,
            // Holds the 4 working characters of the string
            arr = out(reg) _,
            // Temp
            tmp = out(reg) _,
            // Win table for +3 +0 or +6
            win = in(reg) &WIN_TABLE,
            // The length of the string, also the index in the loop
            len = inout(reg) input.len() => _,
            // The total sum
            sum = out(reg) sum,
        };
    };

    println!("Assembly found: {sum}");
}

fn assembly2(input: &'static str) {
    let sum: u64;
    const WIN_TABLE: [u8; 3] = [0, 3, 6];

    unsafe {
        asm! {
            "mov {sum}, 0",
            // Setup length
            "shr {len}, 2",
            "2:",
            // Store 4 chars in {str}
            "mov {arr:e}, [{str} + {len} * 4 - 4]",
            // Mask space and newline
            "sub {arr}, 0x00580041",
            "mov {tmp}, 0x0000000000FF00FF",
            "and {arr}, {tmp}",
            // Use the sum to calc
            // how much we get for our move.
            // A X => 0 => 3
            // B X => 1
            // C X => 2
            // A Y => 1
            // B Y => 2
            // C Y => 3
            // A Z => 2
            // B Z => 3
            // C Z => 4 => 0
            "mov {tmp}, 0",
            "mov {tmp:l}, {arr:l}",
            "shr {arr}, 16",
            "add {tmp}, {arr}",
            // Invert lowest case
            "jnz 3f",
            "add {tmp}, 3",
            "3:",
            // Invert Top Case
            "and {tmp}, 3",
            "jnz 3f",
            "inc {tmp}",
            "3:",
            // Add to sum
            "add {sum}, {tmp}",
            // Add win bonus
            "mov {tmp:l}, [{win} + {arr}]",
            "and {tmp}, 0xFF",
            "add {sum}, {tmp}",
            // Loop
            "dec {len}",
            "jnz 2b",
            // Holds the pointer to the string
            str = inout(reg) input as *const str as *const u8 => _,
            // Holds the 4 working characters of the string
            arr = out(reg) _,
            // Temp
            tmp = out(reg) _,
            // Win table for +3 +0 or +6
            win = in(reg) &WIN_TABLE,
            // The length of the string, also the index in the loop
            len = inout(reg) input.len() => _,
            // The total sum
            sum = out(reg) sum,
        };
    };

    println!("Assembly found: {sum}");
}

fn part1(input: &'static str) {
    let sum: i32 = input
        .lines()
        .map(|line| match line.chars().collect::<Vec<char>>()[..] {
            ['A', ' ', 'X', ..] => 1 + 3,
            ['A', ' ', 'Y', ..] => 2 + 6,
            ['A', ' ', 'Z', ..] => 3 + 0,
            ['B', ' ', 'X', ..] => 1 + 0,
            ['B', ' ', 'Y', ..] => 2 + 3,
            ['B', ' ', 'Z', ..] => 3 + 6,
            ['C', ' ', 'X', ..] => 1 + 6,
            ['C', ' ', 'Y', ..] => 2 + 0,
            ['C', ' ', 'Z', ..] => 3 + 3,
            ref chrs => {
                println!("Error: {chrs:?}");
                -10000
            }
        })
        .sum();

    println!("Final score: {sum}");
}

fn part2(input: &'static str) {
    let sum: i32 = input
        .lines()
        .map(|line| match line.chars().collect::<Vec<char>>()[..] {
            ['A', ' ', 'X', ..] => 0 + 3,
            ['A', ' ', 'Y', ..] => 3 + 1,
            ['A', ' ', 'Z', ..] => 6 + 2,
            ['B', ' ', 'X', ..] => 0 + 1,
            ['B', ' ', 'Y', ..] => 3 + 2,
            ['B', ' ', 'Z', ..] => 6 + 3,
            ['C', ' ', 'X', ..] => 0 + 2,
            ['C', ' ', 'Y', ..] => 3 + 3,
            ['C', ' ', 'Z', ..] => 6 + 1,
            ref chrs => {
                println!("Error: {chrs:?}");
                -10000
            }
        })
        .sum();

    println!("Final score: {sum}");
}
