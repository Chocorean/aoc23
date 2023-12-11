use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

/// Look for empty lines, then columns in the input data
/// Return [[index of empty rows], [index of empty columns]]
fn get_extension(content: &str) -> Vec<Vec<usize>> {
    let mut line_length = 0;
    let mut lines = vec![];
    for (i, line) in content.split('\n').enumerate() {
        // Saving length size for later
        if line_length == 0 {
            line_length = line.len();
        }

        if line.chars().all(|c| c == '.') {
            lines.push(i);
        }
    }
    let mut cols = vec![];
    for j in 0..line_length {
        if content
            .split('\n')
            .all(|line| line.chars().nth(j).unwrap() == '.')
        {
            cols.push(j);
        }
    }
    vec![lines, cols]
}

fn get_galaxies(content: &str, extension: Vec<Vec<usize>>, factor: usize) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for (i, line) in content.split('\n').enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                // Computing offset due to extension
                let mut ext_i = extension
                    .get(0)
                    .unwrap()
                    .iter()
                    .filter(|&x| *x < i)
                    .collect::<Vec<&usize>>()
                    .len();
                let mut ext_j = extension
                    .get(1)
                    .unwrap()
                    .iter()
                    .filter(|&x| *x < j)
                    .collect::<Vec<&usize>>()
                    .len();
                if ext_i != 0 {
                    ext_i = ext_i * (factor - 1);
                }
                if ext_j != 0 {
                    ext_j = ext_j * (factor - 1);
                }
                galaxies.push((i + ext_i, j + ext_j));
            }
        }
    }
    galaxies
}

/// We just count the differrence of positions
fn ez_shortest(galaxies: Vec<(usize, usize)>) -> Vec<usize> {
    let mut distances = vec![];
    for (i, galaxy) in galaxies.iter().enumerate() {
        for (j, other) in galaxies.iter().enumerate() {
            if j <= i {
                continue;
            }
            distances.push(
                usize::max(galaxy.0, other.0) - usize::min(galaxy.0, other.0)
                    + usize::max(galaxy.1, other.1)
                    - usize::min(galaxy.1, other.1),
            )
        }
    }
    distances
}

fn step1(content: &str) -> usize {
    let extension = get_extension(&content);
    let galaxies = get_galaxies(content, extension, 2);
    ez_shortest(galaxies)
        .iter()
        .map(|&x| x)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn step2(content: &str) -> usize {
    let extension = get_extension(&content);
    let galaxies = get_galaxies(content, extension, 1000000);
    ez_shortest(galaxies)
        .iter()
        .map(|&x| x)
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_get_extension() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(get_extension(&content), vec![vec![3, 7], vec![2, 5, 8]]);
    }

    #[test]
    fn test_get_galaxies() {
        let content = fs::read_to_string("test.txt").unwrap();
        let extension = get_extension(&content);
        assert_eq!(
            get_galaxies(&content, extension, 2),
            vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5)
            ]
        );
    }

    #[test]
    fn test_step1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content), 374)
    }

    #[test]
    fn test_expend_larger() {
        let content = fs::read_to_string("test.txt").unwrap();
        let extension = get_extension(&content);
        let mut galaxies = get_galaxies(&content, extension.clone(), 10);
        let mut sum = ez_shortest(galaxies)
            .iter()
            .map(|&x| x)
            .reduce(|a, b| a + b)
            .unwrap();
        assert_eq!(sum, 1030);
        galaxies = get_galaxies(&content, extension, 100);
        sum = ez_shortest(galaxies)
            .iter()
            .map(|&x| x)
            .reduce(|a, b| a + b)
            .unwrap();
        assert_eq!(sum, 8410);
    }
}
