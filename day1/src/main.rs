use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = content.split('\n').collect();
    step1(&lines);
    step2(lines);
}

fn step2(lines: Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let first = get_digit(line, true);
        let last = get_digit(line, false);
        sum += first * 10 + last;
    }
    println!("{sum}");
}

/// Read the input, but also consider the letters. We slowly inspect the string until
/// we find a digit, or until the input string `s` starts with a spelt digit i.e. "eight"
/// `first` controls if we read from left to right (1st digit) or right to left (last).
fn get_digit(s: &str, first: bool) -> u32 {
    let mut chars = s.chars();
    let candidate = if first { chars.next() } else { chars.last() };

    if let Some(nb) = candidate.unwrap_or(' ').to_digit(10) {
        nb
    } else if let Some(nb) = NUMBERS.iter().enumerate().find_map(|(i, &n)| {
        if s.starts_with(n) && first || s.ends_with(n) && !first {
            Some((i + 1) as u32)
        } else {
            None
        }
    }) {
        nb
    } else if first {
        get_digit(&s[1..], first)
    } else {
        get_digit(&s[..s.len() - 1], first)
    }
}

/// Look for the first and last digit in the text, combine them,
/// and add that value to the sum.
fn step1(lines: &Vec<&str>) {
    let mut sum = 0;
    for line in lines {
        let mut nb = 0;
        // first digit
        for char in line.chars() {
            if char.is_numeric() {
                nb += 10 * String::from(char).parse::<u32>().unwrap();
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                nb += String::from(char).parse::<u32>().unwrap();
                break;
            }
        }
        sum += nb
    }
    println!("{sum}");
}
