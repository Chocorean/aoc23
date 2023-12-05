use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut location = step1(&content);
    println!("{location}");
    location = step2(content);
    println!("{location}");
}

fn step2(content: String) -> u64 {
    let lines: Vec<&str> = content.split('\n').collect();
    let first_line = *lines.first().unwrap();

    let nbs: Vec<u64> = first_line
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut seeds_data = vec![];
    for i in 0..nbs.len() / 2 {
        let start = *nbs.get(i * 2).unwrap();

        seeds_data.push((start..start + *nbs.get(i * 2 + 1).unwrap(), false));
    }

    for (i, &line) in lines.iter().enumerate() {
        if line.contains(" map:") {
            let mut j = i + 1;
            let mut nb_line = *lines.get(j).unwrap();
            // For each line of values below a 'map' title
            while !nb_line.is_empty() {
                // Reading values
                let values = nb_line
                    .split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let dst = *values.first().unwrap();
                let src = *values.get(1).unwrap();
                let rng = *values.get(2).unwrap();

                // Updating seeds_data
                let mut data_len = seeds_data.len();
                let mut k = 0;
                while k < data_len {
                    // cloning since cannot borrow immut and mut (later)
                    let data = seeds_data.get(k).unwrap().clone();
                    // if data not updated yet
                    if !data.1 {
                        if src <= data.0.start && src + rng >= data.0.end {
                            // if the new range is larger than the current, update all
                            let data_mut = seeds_data.get_mut(k).unwrap();
                            (data_mut.0, data_mut.1) =
                                (data.0.start + dst - src..data.0.end + dst - src, true);
                        } else if data.0.contains(&src) && data.0.contains(&(src + rng - 1)) {
                            // if new range is included in old one, split in three
                            seeds_data.push((data.0.start..src, false));
                            seeds_data.push((src + rng..data.0.end, false));
                            let data_mut = seeds_data.get_mut(k).unwrap();
                            (data_mut.0, data_mut.1) = (dst..dst + rng, true);
                            data_len += 2; // we pushed two new ranges
                        } else if data.0.contains(&src) {
                            // else if new range is partially included to the end, split in two
                            seeds_data.push((data.0.start..src, false));
                            let data_mut = seeds_data.get_mut(k).unwrap();
                            (data_mut.0, data_mut.1) = (dst..dst + data.0.end - src, true);
                            data_len += 1; // we pushed a new range
                        } else if data.0.contains(&(src + rng - 1)) {
                            // else if new range is partially included from the beginning, split in two
                            seeds_data.push((src + rng..data.0.end, false));
                            let data_mut = seeds_data.get_mut(k).unwrap();
                            (data_mut.0, data_mut.1) = (dst + data.0.start - src..dst + rng, true);
                            data_len += 1; // we pushed a new range
                        }
                        // else: the ranges are mutually exclusive, skipping
                    }
                    k += 1;
                }
                j += 1;
                if let Some(&line) = lines.get(j) {
                    nb_line = line;
                } else {
                    break;
                }
            }
            // Resetting .1
            for j in 0..seeds_data.len() {
                let data = seeds_data.get_mut(j).unwrap();
                data.1 = false;
            }
        }
    }
    seeds_data
        .iter()
        .map(|(d, _)| d.start)
        .reduce(u64::min)
        .unwrap()
}

fn step1(content: &str) -> u64 {
    let lines: Vec<&str> = content.split('\n').collect();
    let first_line = *lines.first().unwrap();
    // Correspond to (value, is_edited)
    // For each new map, for each mapping, we update the relevant seeds
    //   and set .1 to `true` to indicate is has already been updated.
    // After the map is read, we switch all the .1 to `false`.
    let mut seeds_data: Vec<(u64, bool)> = first_line
        .split(' ')
        .skip(1)
        .map(|s| (s.parse::<u64>().unwrap(), false))
        .collect();

    for (i, &line) in lines.iter().enumerate() {
        if line.contains(" map:") {
            let mut j = i + 1;
            let mut nb_line = *lines.get(j).unwrap();
            // For each line of values below a 'map' title
            while !nb_line.is_empty() {
                // Reading values
                let values = nb_line
                    .split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let dst = *values.first().unwrap();
                let src = *values.get(1).unwrap();
                let rng = *values.get(2).unwrap();
                // Updating seeds_data
                for k in 0..seeds_data.len() {
                    let data = seeds_data.get_mut(k).unwrap();
                    if !data.1 {
                        // if data not updated yet
                        if (src..src + rng).contains(&data.0) {
                            // if data is included in src range
                            data.1 = true;
                            let difference = data.0 - src;
                            data.0 = dst + difference;
                        }
                    }
                }
                j += 1;
                if let Some(&line) = lines.get(j) {
                    nb_line = line;
                } else {
                    break;
                }
            }
            // Resetting .1
            for j in 0..seeds_data.len() {
                let data = seeds_data.get_mut(j).unwrap();
                data.1 = false;
            }
        }
    }
    seeds_data.iter().map(|(d, _)| *d).reduce(u64::min).unwrap()
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_step1() {
        use crate::step1;

        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(35, step1(&content));
    }

    #[test]
    fn test_step2() {
        use crate::step2;

        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(46, step2(content));
    }
}
