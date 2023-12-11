use std::fs;

fn main() {
    let content = fs::read_to_string("test.txt").unwrap();
    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

fn step1(content: &str) -> u64 {
    0
}

fn step2(content: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use crate::*;

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
