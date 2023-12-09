use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

/// Generate the next sequence
fn next_seq(vec: &Vec<i64>) -> Vec<i64> {
    let mut next = vec![];
    for i in 0..vec.len() - 1 {
        next.push(vec.get(i + 1).unwrap() - vec.get(i).unwrap())
    }
    next
}

/// From a single seq, generate all the smaller ones
fn build_seqs(vec: Vec<i64>) -> Vec<Vec<i64>> {
    let mut next = next_seq(&vec);
    let mut seqs = vec![vec, next.clone()];
    while !next.iter().all(|v| *v == 0) {
        next = next_seq(seqs.get(seqs.len() - 1).unwrap());
        seqs.push(next.clone());
    }
    seqs
}

/// Compute the next number of a sequence
fn next_number(vec: Vec<i64>) -> i64 {
    let mut seqs = build_seqs(vec);
    seqs.reverse();
    seqs.iter()
        .map(|v| *v.last().unwrap())
        .reduce(|v0, v1| v0 + v1)
        .unwrap()
}

fn step1(content: &str) -> i64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    let seqs = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|nb| nb.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    seqs.iter()
        .map(|v| next_number(v.to_vec()))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn step2(content: &str) -> i64 {
    let lines = content.split('\n').collect::<Vec<&str>>();
    let seqs = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|nb| nb.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect::<Vec<Vec<i64>>>();
    seqs.iter()
        .map(|v| next_number(v.to_vec()))
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::*;

    #[test]
    fn test_next_seq() {
        assert_eq!(next_seq(&vec![0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
        assert_eq!(next_seq(&vec![1, 3, 6, 10, 15, 21]), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_build_seqs() {
        assert_eq!(
            build_seqs(vec![0, 3, 6, 9, 12, 15]),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0]
            ]
        );
    }

    #[test]
    fn test_step1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content), 114)
    }

    #[test]
    fn test_step2() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step2(&content), 2)
    }
}
