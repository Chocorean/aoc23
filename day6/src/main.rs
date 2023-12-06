use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

fn step2(content: &str) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = content
        .split('\n')
        .map(|s| s.replace(' ', ""))
        .collect::<Vec<String>>();

    let time = re
        .find(lines.first().unwrap())
        .unwrap()
        .as_str()
        .parse::<u64>()
        .unwrap();
    let record = re
        .find(lines.get(1).unwrap())
        .unwrap()
        .as_str()
        .parse::<u64>()
        .unwrap();

    let mut winnable = 0;
    // compute each scenario and save the number of winnable ones
    for j in 0..time {
        let speed = j;
        let remaining_time = time - j;
        let dist = remaining_time * speed;
        if dist > record {
            winnable += 1;
        }
    }
    winnable
}

fn step1(content: &str) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let lines = content.split('\n').collect::<Vec<&str>>();

    let mut times = vec![];
    for m in re.find_iter(lines.first().unwrap()) {
        times.push(m.as_str().parse::<u64>().unwrap());
    }

    let mut dists = vec![];
    for m in re.find_iter(lines.get(1).unwrap()) {
        dists.push(m.as_str().parse::<u64>().unwrap());
    }

    // for each contest
    let mut result = 1;
    for i in 0..times.len() {
        let mut winnable = 0;
        // compute each scenario and save the number of winnable ones
        let time = *times.get(i).unwrap();
        let record = *dists.get(i).unwrap();
        for j in 0..time {
            let speed = j;
            let remaining_time = time - j;
            let dist = remaining_time * speed;
            if dist > record {
                winnable += 1;
            }
        }
        result *= winnable;
    }
    result
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_step1() {
        use crate::step1;
        let content: String = fs::read_to_string("test.txt").unwrap();

        assert_eq!(288, step1(&content));
    }

    #[test]
    fn test_step2() {
        use crate::step2;
        let content: String = fs::read_to_string("test.txt").unwrap();

        assert_eq!(71503, step2(&content));
    }
}
