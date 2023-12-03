use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    solve(&content, false);
    solve(&content, true);
}

// a &str is a symbol if it is different than a digit or than a dot '.'
fn is_symbol(x: usize, y: usize, lines: &Vec<&str>) -> bool {
    let line = *lines.get(x).unwrap();
    println!("{line} {} {y}", line.len());
    let char = line.get(y as usize..(y + 1) as usize).unwrap(); // works since we know the input

    // println!("  {char}");
    if char.parse::<u8>().is_ok() {
        return false;
    }
    match char {
        "." => false,
        _ => true, // symbol like ?, #, ...
    }
}

struct Part {
    nb: u64,
    pos: (i32, i32),
}

impl Part {
    pub fn new(nb: u64, pos: (i32, i32)) -> Self {
        Self { nb, pos }
    }
}

fn solve(content: &String, part2: bool) {
    let nb_regex = Regex::new(r"(\d+)").unwrap();

    let mut sum = 0;
    let mut parts = HashMap::<(i32, i32), Vec<u64>>::new();
    let mut last_gear = (0, 0);

    let lines = content.split('\n').collect::<Vec<&str>>();

    // for each number identified by the regex capture,
    //  we inspect the box around it and look for a symbol
    for (line_id, &line) in lines.iter().enumerate() {
        let caps = nb_regex.captures_iter(line);
        for cap in caps {
            let mut i = 0;
            while let Some(r#match) = cap.get(i) {
                // println!(
                //     "#{line_id} Inspecting {} start {} end {}",
                //     r#match.as_str(),
                //     r#match.start(),
                //     r#match.end(),
                // );
                // inspect all the cells around the number
                let mut candidate = false;
                for x in i32::max(0, line_id as i32 - 1)
                    ..i32::min(line_id as i32 + 2, lines.len() as i32)
                {
                    for y in i32::max(0, r#match.start() as i32 - 1)
                        ..i32::min(r#match.end() as i32 + 1, line.len() as i32)
                    {
                        if is_symbol(x as usize, y as usize, &lines) {
                            candidate = true;
                            last_gear = (x, y);
                            break;
                        }
                    }
                    if candidate {
                        break;
                    }
                }
                if candidate {
                    let id = r#match.as_str().parse::<u64>().unwrap();
                    if !part2 {
                        sum += id;
                        // println!("  #{line_id} adding {id}");
                    } else {
                        let mut opt = parts.get_mut(&last_gear);
                        if let Some(ref mut v) = opt {
                            v.push(id)
                        } else {
                            parts.insert(last_gear, vec![id]);
                        }
                    }
                }
                i += 2; // my regex sucks and each get counter twice lol
            }
        }
    }
    if !part2 {
        println!("{sum}");
    } else {
        let sum: u64 = parts
            .values()
            .filter(|&v| v.len() == 2)
            .map(|v| v.iter().map(|&n| n).reduce(|a, b| a * b).unwrap())
            .sum();
        println!("{sum}");
    }
}
