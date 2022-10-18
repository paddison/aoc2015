use std::collections::HashSet;

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| c.into()).collect()
}

pub fn get_solution_1() -> usize {
    let dirs = parse(include_str!("../data/d3.txt"));
    let mut cur_pos = (0, 0); // x, y
    let mut all_pos = HashSet::from([cur_pos]);
    for dir in dirs {
        cur_pos = match dir {
            Dir::Up => (cur_pos.0, cur_pos.1 + 1),
            Dir::Down => (cur_pos.0, cur_pos.1 - 1),
            Dir::Right => (cur_pos.0 + 1, cur_pos.1),
            Dir::Left => (cur_pos.0 - 1, cur_pos.1),
        };
        all_pos.insert(cur_pos);
    }

    all_pos.len()
}

pub fn get_solution_2() -> usize {
    let dirs = parse(include_str!("../data/d3.txt"));
    let mut santa_pos = (0, 0); // x, y
    let mut robo_santa_pos = (0, 0);
    let mut all_pos = HashSet::from([santa_pos]);
    for (i, dir) in dirs.iter().enumerate() {
        let pos = if i % 2 == 0 { &mut santa_pos } else { &mut robo_santa_pos };
        *pos = match dir {
            Dir::Up => (pos.0, pos.1 + 1),
            Dir::Down => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0 - 1, pos.1),
        };
        all_pos.insert(*pos);
    }

    all_pos.len()
}

enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => panic!("Invalid direction."),
        }
    }
}

