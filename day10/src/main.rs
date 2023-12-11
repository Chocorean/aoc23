use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut res = step1(&content);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Top,
    Bottom,
}

/// Helper for creating list of `Direction`s
macro_rules! d {
    ($($x:tt), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Direction::$x);
            )*
            temp_vec
        }
    };
}

/// Cell will be updated eventually if its kind is `Start`
/// If `is_loop` is false, it will never be considered in the computation
struct Cell {
    kind: char,
    visited: bool,
}

impl Cell {
    pub fn new(kind: char) -> Self {
        Self {
            kind,
            visited: false,
        }
    }
    pub fn connect_to(&self) -> Vec<Direction> {
        match self.kind {
            '-' => d!(Left, Right),
            '|' => d!(Top, Bottom),
            'J' => d!(Left, Top),
            'F' => d!(Bottom, Right),
            '7' => d!(Left, Bottom),
            'L' => d!(Top, Right),
            _ => vec![], // `Start` and `Ground`
        }
    }
}

struct Map {
    map: Vec<Vec<Cell>>,
    start: (usize, usize),
}

impl Map {
    pub fn new(input: &str) -> Self {
        // Load map and save Start position
        let mut map = vec![];
        let mut start = Default::default();
        for (i, line) in input.split('\n').enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().into_iter().enumerate() {
                row.push(Cell::new(c));
                if c == 'S' {
                    start = (i, j);
                }
            }
            map.push(row);
        }
        let mut map = Self { map, start };
        map.update_start();
        map
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let res = self.map.get(x);
        if res.is_none() {
            return None;
        }
        res.unwrap().get(y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let res = self.map.get_mut(x);
        if res.is_none() {
            return None;
        }
        res.unwrap().get_mut(y)
    }

    fn next(&self, current: (usize, usize), old: Option<(usize, usize)>) -> (usize, usize) {
        let current_cell = self.get(current.0, current.1).unwrap();
        let dirs = current_cell.connect_to();
        let new_dir = match old {
            None => *dirs.first().unwrap(),
            Some((x, y)) => {
                let from = if x == current.0 {
                    if y < current.1 {
                        Direction::Left
                    } else {
                        Direction::Right
                    }
                } else {
                    assert_eq!(y, current.1);
                    if x < current.0 {
                        Direction::Top
                    } else {
                        Direction::Bottom
                    }
                };
                *dirs.iter().filter(|&d| d != &from).next().unwrap()
            }
        };
        match new_dir {
            Direction::Right => (current.0, current.1 + 1),
            Direction::Left => (current.0, current.1 - 1),
            Direction::Top => (current.0 - 1, current.1),
            Direction::Bottom => (current.0 + 1, current.1),
        }
    }

    /// Update S cell to the appropriate pipe, and mark it for the loop.
    fn update_start(&mut self) {
        let start_x = self.start.0;
        let start_y = self.start.1;

        let neighbors = get_4_neighbors(start_x, start_y);
        assert!(neighbors.len() <= 4);

        let mut dirs = vec![];
        for &(x, y) in neighbors.iter() {
            if let Some(cell) = self.get(x, y) {
                // Check the axis, and direction accordlingly
                if x == start_x {
                    // Aligned horizontally so check Left and Right
                    if y > start_y {
                        // Right
                        if cell.connect_to().contains(&Direction::Left) {
                            dirs.push(Direction::Right);
                        }
                    } else {
                        // Left
                        if cell.connect_to().contains(&Direction::Right) {
                            dirs.push(Direction::Left);
                        }
                    }
                } else if y == start_y {
                    // Aligned vertically so check Bottom and Top
                    if x > start_x {
                        // Bottom
                        if cell.connect_to().contains(&Direction::Top) {
                            dirs.push(Direction::Bottom);
                        }
                    } else {
                        // Top
                        if cell.connect_to().contains(&Direction::Bottom) {
                            dirs.push(Direction::Top);
                        }
                    }
                }
            }
        }
        // If not, something wrong happened
        assert_eq!(dirs.len(), 2);

        // Update Start cell with the proper pipe kind
        let start_cell = self.get_mut(start_x, start_y).unwrap();
        start_cell.visited = true;
        if dirs.contains(&Direction::Bottom) && dirs.contains(&Direction::Top) {
            start_cell.kind = '|';
        } else if dirs.contains(&Direction::Bottom) && dirs.contains(&Direction::Left) {
            start_cell.kind = '7';
        } else if dirs.contains(&Direction::Bottom) && dirs.contains(&Direction::Right) {
            start_cell.kind = 'F';
        } else if dirs.contains(&Direction::Top) && dirs.contains(&Direction::Left) {
            start_cell.kind = 'J';
        } else if dirs.contains(&Direction::Top) && dirs.contains(&Direction::Right) {
            start_cell.kind = 'L';
        } else if dirs.contains(&Direction::Right) && dirs.contains(&Direction::Left) {
            start_cell.kind = '-';
        } else {
            panic!("Start does not have two ends !");
        }
    }

    pub fn get_loop(&self) -> Vec<(usize, usize)> {
        let mut seen = vec![self.start];
        let start_pos = self.start;
        let mut last_pos = start_pos.clone();
        let mut next_pos = self.next(start_pos, None);
        while next_pos != start_pos {
            seen.push(next_pos);
            (last_pos, next_pos) = (next_pos, self.next(next_pos, Some(last_pos)));
        }
        seen
    }

    pub fn inside(&self) -> usize {
        0
    }
}

/// Return a list of neighbors of the cell(x,y), at most 4 neighbors
fn get_4_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];
    // Safe to add these two since it won't overflow
    v.push((x, y + 1));
    v.push((x + 1, y));
    // These two may underflow so we do differently
    if x > 0 {
        v.push((x - 1, y))
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    v
}

fn step1(content: &str) -> usize {
    let map = Map::new(content);
    let pipe_loop = map.get_loop();
    (pipe_loop.len() + 1) / 2
}

fn step2(content: &str) -> usize {
    let map = Map::new(content);
    let pipe_loop = map.get_loop();
    let mut inside_pos = vec![];
    for i in 0..map.map.len() - 1 {
        let mut inside = false;
        for j in 0..map.map.get(0).unwrap().len() - 1 {
            // for each cell, if it is on the loop there are two scenarios
            // 1. it connects to its right: we are not inside
            // 2. it does not connect to right
            if pipe_loop.contains(&(i, j)) {
                let cell = map.get(i, j).unwrap();
                if ['|', 'F', '7'].contains(&cell.kind) {
                    inside = !inside;
                }
            } else {
                if inside {
                    inside_pos.push((i, j));
                }
            }
        }
    }
    inside_pos.len()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::*;

    #[test]
    fn test_connect_to() {
        assert_eq!(Cell::new('-').connect_to(), d![Left, Right]);
        assert_eq!(Cell::new('|').connect_to(), d![Top, Bottom]);
        assert_eq!(Cell::new('F').connect_to(), d![Bottom, Right]);
        assert_eq!(Cell::new('J').connect_to(), d![Left, Top]);
        assert_eq!(Cell::new('7').connect_to(), d![Left, Bottom]);
        assert_eq!(Cell::new('L').connect_to(), d![Top, Right]);
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(get_4_neighbors(0, 0), vec![(0, 1), (1, 0)]);
        assert_eq!(get_4_neighbors(4, 4), vec![(4, 5), (5, 4), (3, 4), (4, 3)]);
        assert_eq!(get_4_neighbors(1, 2), vec![(1, 3), (2, 2), (0, 2), (1, 1)]);
    }

    #[test]
    fn test_next() {
        let content = fs::read_to_string("test.txt").unwrap();
        let map = Map::new(&content);
        let mut next_pos = map.next((1, 1), None);
        assert_eq!(next_pos, (2, 1)); // Bottom is the first of F [1,1]
        next_pos = map.next(next_pos, Some((1, 1)));
        assert_eq!(next_pos, (3, 1)); // |
        next_pos = map.next(next_pos, Some((2, 1)));
        assert_eq!(next_pos, (3, 2)); // L
    }

    #[test]
    fn test_step1_1() {
        let content = fs::read_to_string("test.txt").unwrap();
        assert_eq!(step1(&content), 4)
    }

    #[test]
    fn test_step1_2() {
        let content = fs::read_to_string("test2.txt").unwrap();
        assert_eq!(step1(&content), 8)
    }

    #[test]
    fn test_step2_1() {
        let content = fs::read_to_string("test3.txt").unwrap();
        assert_eq!(step2(&content), 4)
    }

    #[test]
    fn test_step2_2() {
        let content = fs::read_to_string("test4.txt").unwrap();
        assert_eq!(step2(&content), 4)
    }

    #[test]
    fn test_step2_3() {
        let content = fs::read_to_string("test5.txt").unwrap();
        assert_eq!(step2(&content), 8)
    }
}
