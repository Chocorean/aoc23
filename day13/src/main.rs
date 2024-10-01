use std::{
    fs,
    str::Bytes,
    sync::{Arc, Mutex},
};

fn main() {
    let content = fs::read_to_string("test.txt").unwrap();
    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

struct Pattern<'a> {
    data: &'a str,
    width: usize,
    length: usize,
}

impl<'a> Pattern<'a> {
    pub fn new(data: &'a str) -> Self {
        let lines: Vec<_> = data.split('\n').collect();
        let length = lines.len();
        let width = lines.first().unwrap().len();
        Self {
            data,
            width,
            length,
        }
    }

    pub fn get_row(&self, i: usize) -> &'a str {
        self.data.split('\n').skip(i).next().unwrap()
    }

    pub fn get_col(&self, i: usize) -> String {
        self.data
            .split('\n')
            .map(|line| line[i..i + 1].to_string())
            .reduce(|mut a, b| {
                a.push_str(&b);
                a
            })
            .unwrap()
    }

    pub fn sym_horiz(&self, index: usize) -> Vec<usize> {
        let col = self.get_col(index);
        sym(&col)
    }

    pub fn sym_vert(&self, index: usize) -> Vec<usize> {
        let row = self.get_row(index);
        sym(row)
    }
}

fn sym(line: &str) -> Vec<usize> {
    let line = String::from(line);
    for i in 0..line.len() - 1 {
        let left = line.get(i..i + 1).unwrap();
        let right = line.get(i + 1..i + 2);
    }
    vec![]
}

fn step1(content: &str) -> u64 {
    let split = content.split("\n\n");
    let patterns = content
        .split("\n\n")
        .map(|data| Pattern::new(data))
        .collect::<Vec<Pattern>>();
    let sum = Arc::new(Mutex::new(0));
    let value = *sum.lock().unwrap();
    value
}

fn step2(content: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_get_row() {
        let content = fs::read_to_string("test.txt").unwrap();
        let pattern = Pattern::new(content.split("\n\n").next().unwrap());
        assert_eq!(pattern.get_row(3), "##......#");
        assert_eq!(pattern.get_row(6), "#.#.##.#.");
    }

    #[test]
    fn test_get_col() {
        let content = fs::read_to_string("test.txt").unwrap();
        let pattern = Pattern::new(content.split("\n\n").next().unwrap());
        assert_eq!(&pattern.get_col(3), "#....#.");
        assert_eq!(&pattern.get_col(6), "#....#.");
    }

    #[test]
    fn test_sym() {
        let content = fs::read_to_string("test.txt").unwrap();
        let mut patterns_str = content.split("\n\n");
        let pattern_a = Pattern::new(patterns_str.next().unwrap());
        let pattern_b = Pattern::new(patterns_str.next().unwrap());
        assert_eq!(pattern_a.sym_horiz(0), vec![]);
        assert_eq!(pattern_a.sym_horiz(1), vec![]);
        assert_eq!(pattern_a.sym_vert(2), vec![4]);
        assert_eq!(pattern_a.sym_vert(3), vec![4]);
        assert_eq!(pattern_b.sym_horiz(0), vec![3]);
        assert_eq!(pattern_b.sym_horiz(1), vec![3]);
        assert_eq!(pattern_b.sym_vert(2), vec![]);
        assert_eq!(pattern_b.sym_vert(3), vec![]);
    }

    #[test]
    fn test_step1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content), 0)
    }

    #[test]
    fn test_step2() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step2(&content), 0)
    }
}
