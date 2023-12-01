use itertools::Itertools;
use std::arch::asm;

fn main() {
    let input = include_str!("input.txt");

    assembly1(input);
    part1(input);
    assembly2(input);
    part2(input);
}

fn assembly1(input: &'static str) {
    let mut sum = 0usize;

    unsafe {
        asm! {
            // Init
            "dec {len}",
            "mov {ind}, 0",
            "2:",
            // Stuff
            "mov {line}, {ind}",
            // Find \n
            "3:",
            "inc {line}",
            "cmp byte ptr [{str} + {line}], '\n'",
            "jnz 3b",
            // Find middle
            "sub {line}, {ind}",
            "shr {line}, 1",
            "mov {half}, {line}",
            "add {line}, {ind}",
            // Go through first half
            "mov {temp}, 0",
            "mov {char}, 0",
            "4:",
            "mov {temp:l}, [{str} + {ind}]",
            "sub {temp}, 'A'",
            "mov CL, {temp:l}",
            "mov {temp}, 1",
            "shl {temp}, CL",
            "or {char}, {temp}",
            "inc {ind}",
            "cmp {line}, {ind}",
            "jnz 4b",
            "add {line}, {half}",
            // Go through second half, find first match
            "5:",
            "mov {temp:l}, [{str} + {ind}]",
            "sub {temp}, 'A'",
            "mov CL, {temp:l}",
            "mov {temp}, 1",
            "shl {temp}, CL",
            "inc {ind}",
            "and {temp}, {char}",
            "jz 5b",
            // Get priority
            "mov {temp:l}, [{str} + {ind} - 1]",
            "cmp {temp:l}, 'Z' + 1",
            "jns 6f",
            // Handle upper case
            "sub {temp:l}, 'A' - 27",
            "jmp 7f",
            // Handle lower case
            "6:",
            "sub {temp:l}, 'a' - 1",
            // Add to sum
            "7:",
            "and {temp}, 0xFF",
            "add {sum}, {temp}",
            // For loop
            "mov {ind}, {line}",
            "inc {ind}",
            "cmp {ind}, {len}",
            "js 2b",
            str = in(reg) input as *const str as *const u8,
            len = in(reg) input.len(),
            sum = inout(reg) sum,
            ind = out(reg) _,
            line = out(reg) _,
            temp = out(reg) _,
            char = out(reg) _,
            half = out(reg) _,
            out("cl") _,
            options(pure, readonly, nostack),
        }
    };

    dbg!(sum);
}

fn part1(input: &'static str) {
    let sum: u32 = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(f, b)| f.chars().filter(|c| b.chars().contains(c)).next().unwrap() as u32)
        .map(|c| match c as u8 {
            b'a'..=b'z' => c - 'a' as u32 + 1,
            b'A'..=b'Z' => c - 'A' as u32 + 27,
            _ => panic!("Not supposed to be here..."),
        })
        .sum();

    println!("Sum is {sum}");
}

fn assembly2(input: &'static str) {
    let mut sum = 0usize;

    unsafe {
        asm! {
            // Init
            "dec {len}",
            "mov {ind}, 0",
            // Go through first line
            "2:",
            "mov {temp}, 0",
            "mov {first}, 0",
            "mov {final}, 0",
            "3:",
            "mov {temp:l}, [{str} + {ind}]",
            "cmp {temp:l}, '\n'",
            "jz 4f",
            "sub {temp}, 'A'",
            "mov CL, {temp:l}",
            "mov {temp}, 1",
            "shl {temp}, CL",
            "or {first}, {temp}",
            "inc {ind}",
            "jmp 3b",
            // Go through second line, store all matchs
            "4:",
            "inc {ind}",
            "mov {temp:l}, [{str} + {ind}]",
            "cmp {temp:l}, '\n'",
            "jz 5f",
            "sub {temp}, 'A'",
            "mov CL, {temp:l}",
            "mov {temp}, 1",
            "shl {temp}, CL",
            "and {temp}, {first}",
            "or {final}, {temp}",
            "jmp 4b",
            // Go through third line, find first match
            "5:",
            "inc {ind}",
            "mov {temp:l}, [{str} + {ind}]",
            "sub {temp}, 'A'",
            "mov CL, {temp:l}",
            "mov {temp}, 1",
            "shl {temp}, CL",
            "and {temp}, {final}",
            "jz 5b",
            // Get priority
            "mov {temp:l}, [{str} + {ind}]",
            "cmp {temp:l}, 'Z' + 1",
            "jns 6f",
            // Handle upper case
            "sub {temp:l}, 'A' - 27",
            "jmp 7f",
            // Handle lower case
            "6:",
            "sub {temp:l}, 'a' - 1",
            // Add to sum
            "7:",
            "and {temp}, 0xFF",
            "add {sum}, {temp}",
            // Find end of line
            "8:",
            "inc {ind}",
            "cmp byte ptr [{str} + {ind}], '\n'",
            "jnz 8b",
            "inc {ind}",
            // For loop
            "cmp {ind}, {len}",
            "js 2b",
            str = in(reg) input as *const str as *const u8,
            len = in(reg) input.len(),
            sum = inout(reg) sum,
            ind = out(reg) _,
            temp = out(reg) _,
            first = out(reg) _,
            final = out(reg) _,
            out("cl") _,
            options(pure, readonly, nostack),
        }
    };

    dbg!(sum);
}

fn part2(input: &'static str) {
    let sum: u32 = input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .filter(|d| b.chars().contains(d) && c.chars().contains(d))
                .next()
                .unwrap() as u32
        })
        .map(|c| match c as u8 {
            b'a'..=b'z' => c - 'a' as u32 + 1,
            b'A'..=b'Z' => c - 'A' as u32 + 27,
            _ => panic!("Not supposed to be here..."),
        })
        .sum();

    println!("Sum is {sum}");
}
