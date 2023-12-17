use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn candidates(parts: &str, target: Vec<u64>) -> u64 {
    let mut cands = 0;
    let question_indexes = parts
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '?')
        .map(|(ind, _)| ind)
        .collect::<Vec<_>>();
    let question_count = question_indexes.len();

    let max_cands = 2_u64.pow(question_count as u32);

    // for each candidate
    for i in 0..max_cands {
        let mut cand = String::from(parts);
        // for each ?
        for (ind, &n) in question_indexes.iter().enumerate() {
            // Replace the `n`th ? with the `ind` bit value of `i`q
            let bin = i >> ind & 1;
            let new = if bin == 1 { '#' } else { '.' };
            cand.replace_range(n..n + 1, &new.to_string());
        }
        if group(&cand) == target {
            cands += 1;
        }
    }
    cands
}

/// Given a sequence of '.' and '#', return the groups of '#'
fn group(parts: &str) -> Vec<u64> {
    let mut groups = vec![0];
    let mut last_was_damaged = false;
    for char in parts.chars() {
        if char == '#' {
            last_was_damaged = true;
            *groups.last_mut().unwrap() += 1;
        } else {
            if last_was_damaged {
                last_was_damaged = false;
                groups.push(0);
            }
        }
    }
    groups.into_iter().filter(|&d| d != 0).collect()
}

pub fn step1(content: &str, par: bool) -> u64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    if !par {
        let mut sum = 0;
        for line in lines {
            let line_parts = line.split(' ').collect::<Vec<&str>>();
            let parts = *line_parts.first().unwrap();
            let target = line_parts
                .last()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            sum += candidates(parts, target);
        }
        sum
    } else {
        let sum = Arc::new(Mutex::new(0));
        let clone = Arc::clone(&sum);
        lines.par_iter().for_each(|line| {
            let line_parts = line.split(' ').collect::<Vec<&str>>();
            let parts = *line_parts.first().unwrap();
            let target = line_parts
                .last()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            *clone.lock().unwrap() += candidates(parts, target);
        });
        let value = *(sum.lock().unwrap());
        value
    }
}

pub fn step2(content: &str) -> u64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    let sum = Arc::new(Mutex::new(0));
    let clone = Arc::clone(&sum);

    lines.par_iter().for_each(|line| {
        let line_parts = line.split(' ').collect::<Vec<&str>>();
        let part_fifth = *line_parts.first().unwrap();
        let parts = vec![part_fifth; 5].join("?");

        let mut target = line_parts
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let target_clone = target.clone();
        for _ in 0..4 {
            target.extend_from_slice(&target_clone);
        }
        *clone.lock().unwrap() += candidates(&parts, target);
    });
    let value = *(sum.lock().unwrap());
    value
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::*;

    #[test]
    fn test_group() {
        assert_eq!(group("#.#.###"), vec![1, 1, 3]);
        assert_eq!(group(".#....#...###."), vec![1, 1, 3]);
        assert_eq!(group(".#.###.#.######"), vec![1, 3, 1, 6]);
        assert_eq!(group(".###.##.#."), vec![3, 2, 1]);
    }

    #[test]
    fn test_candidate() {
        assert_eq!(candidates("???.###", vec![1, 1, 3]), 1);
        assert_eq!(candidates(".??..??...?##.", vec![1, 1, 3]), 4);
        assert_eq!(candidates("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]), 1);
        assert_eq!(candidates("????.#...#...", vec![4, 1, 1]), 1);
        assert_eq!(candidates("????.######..#####.", vec![1, 6, 5]), 4);
        assert_eq!(candidates("?###????????", vec![3, 2, 1]), 10);
    }

    #[test]
    fn test_step1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content, false), 21);
    }

    #[test]
    fn test_step1_par() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content, true), 21);
    }

    #[test]
    fn test_step2() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step2(&content), 525152);
    }
}
