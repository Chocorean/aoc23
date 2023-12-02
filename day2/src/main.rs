use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = content.split('\n').collect();

    step1(&lines);
    step2(lines);
}

fn step2(lines: Vec<&str>) {
    let mut sum = 0;
    let color_regex = Regex::new(r"(?<nb>\d+) (?<color>\w+)").unwrap();
    for line in lines {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for cap in color_regex.captures_iter(line) {
            let nb = cap["nb"].parse::<u32>().unwrap();
            let color = &cap["color"];
            match color {
                "red" => {
                    if nb > red {
                        red = nb;
                    }
                }
                "green" => {
                    if nb > green {
                        green = nb;
                    }
                }
                "blue" => {
                    if nb > blue {
                        blue = nb;
                    }
                }
                _ => {}
            }
        }
        sum += red * green * blue;
    }
    println!("{sum}");
}

fn step1(lines: &[&str]) {
    let mut sum = 0;
    let color_regex = Regex::new(r"(?<nb>\d+) (?<color>\w+)").unwrap();
    for (i, &line) in lines.iter().enumerate() {
        let mut candidate = true;
        for cap in color_regex.captures_iter(line) {
            let color = &cap["color"];
            let nb = cap["nb"].parse::<u32>().unwrap();
            match (color, nb) {
                ("red", x) if x > 12 => {
                    candidate = false;
                    break;
                }
                ("green", x) if x > 13 => {
                    candidate = false;
                    break;
                }
                ("blue", x) if x > 14 => {
                    candidate = false;
                    break;
                }
                (_, _) => {}
            }
        }
        if candidate {
            sum += i + 1;
        }
    }
    println!("{sum}");
}
