use num::integer::lcm;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

struct Path<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Path<'a> {
    pub fn new(line: &'a str) -> Self {
        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        Self { name, left, right }
    }
}

struct Map<'a> {
    paths: Vec<Path<'a>>,
}

impl<'a> Map<'a> {
    pub fn new(list: &Vec<&'a str>) -> Self {
        let mut paths = vec![];
        for &path in list.iter() {
            let path = Path::new(path);
            paths.push(path);
        }
        Self { paths }
    }

    pub fn update<'b>(&'a self, current: &'a str, inst: &'b str) -> &'a str {
        let path = self
            .paths
            .iter()
            .filter(|&p| p.name == current)
            .next()
            .unwrap();
        if inst == "L" {
            path.left
        } else {
            path.right
        }
    }
}

fn step1(content: &str) -> u64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    let instructions = *lines.first().unwrap();

    let map = Map::new(&lines.iter().skip(2).map(|&p| p).collect::<Vec<&str>>());
    let mut current_pos = "AAA";
    let mut i = 0_usize;
    let mut cpt = 0;
    while current_pos != "ZZZ" {
        let inst = instructions
            .chars()
            .nth(i)
            .unwrap_or_else(|| {
                i = 0;
                instructions.chars().nth(i).unwrap()
            })
            .to_string();
        current_pos = map.update(current_pos, &inst);
        i += 1;
        cpt += 1;
    }
    cpt
}

fn step2(content: &str) -> u64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    let instructions = *lines.first().unwrap();

    let paths = lines.iter().skip(2).map(|&p| p).collect::<Vec<&str>>();
    let map = Map::new(&paths);
    let current_pos = map
        .paths
        .iter()
        .filter(|&p| p.name.ends_with('A'))
        .map(|p| p.name)
        .collect::<Vec<&str>>();
    let mut distances = vec![];
    for i in 0..current_pos.len() {
        let mut pos = *current_pos.get(i).unwrap();
        let mut j = 0_usize;
        let mut cpt = 0;
        while !pos.ends_with('Z') {
            let inst = instructions
                .chars()
                .nth(j)
                .unwrap_or_else(|| {
                    j = 0;
                    instructions.chars().nth(j).unwrap()
                })
                .to_string();
            pos = map.update(pos, &inst);
            j += 1;
            cpt += 1;
        }
        distances.push(cpt);
    }
    distances.iter().fold(1_u64, |a, &b| lcm(a, b))
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{step1, step2};

    #[test]
    fn test_step1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content), 2)
    }

    #[test]
    fn test_step1_2() {
        let content = fs::read_to_string("test2.txt").unwrap();
        assert_eq!(step1(&content), 6)
    }

    #[test]
    fn test_step2() {
        let content = fs::read_to_string("test3.txt").unwrap();
        assert_eq!(step2(&content), 6)
    }
}
