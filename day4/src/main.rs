use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    step1(&content);
    step2(content);
}

fn step2(content: String) {
    let regex = Regex::new(r"(?<win>[\d ]+) \| (?<nbs>[\d ]+)").unwrap();
    let lines: Vec<&str> = content.split('\n').collect();
    let mut counts = vec![1; lines.len()];

    for (i, &line) in lines.iter().enumerate() {
        let caps = regex.captures_iter(line);
        for cap in caps {
            let mut copies = 0;

            let win_nbs: Vec<&str> = cap["win"].split(' ').filter(|&x| !x.is_empty()).collect();
            let nbs: Vec<&str> = cap["nbs"].split(' ').filter(|&x| !x.is_empty()).collect();
            for nb in nbs.iter() {
                if win_nbs.contains(nb) {
                    copies += 1;
                }
            }

            let current_copies = *counts.get(i).unwrap();
            for j in 1..(copies + 1) {
                let v = counts.get_mut(i + j).unwrap();
                *v += current_copies;
            }
        }
    }
    let sum = counts.iter().sum::<u32>();
    println!("{sum}");
}

fn increment_value(v: &mut u64) {
    if *v == 0 {
        *v = 1;
    } else {
        *v *= 2;
    }
}

fn step1(content: &str) {
    let regex = Regex::new(r"(?<win>[\d ]+) \| (?<nbs>[\d ]+)").unwrap();
    let lines: Vec<&str> = content.split('\n').collect();

    let mut sum = 0;

    for &line in lines.iter() {
        let caps = regex.captures_iter(line);
        for cap in caps {
            let mut value = 0;

            let win_nbs: Vec<&str> = cap["win"].split(' ').filter(|&x| !x.is_empty()).collect();
            let nbs: Vec<&str> = cap["nbs"].split(' ').filter(|&x| !x.is_empty()).collect();
            for nb in nbs.iter() {
                if win_nbs.contains(nb) {
                    increment_value(&mut value);
                }
            }
            sum += value;
        }
    }
    println!("{sum}");
}

#[cfg(test)]
mod test {
    use crate::increment_value;

    #[test]
    fn test_increment() {
        let mut i = 0;
        increment_value(&mut i);
        assert_eq!(i, 1);
        increment_value(&mut i);
        assert_eq!(i, 2);
        increment_value(&mut i);
        assert_eq!(i, 4);
    }
}
