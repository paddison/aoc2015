fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn get_solution_1() -> isize {
    let mut count: isize = 0;
    for c in parse(include_str!("../data/d1.txt")) {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => panic!("Invalid char"),
        }
    }

    count
}

pub fn get_solution_2() -> usize {
    let mut count: isize = 0;
    for (i, c) in parse(include_str!("../data/d1.txt")).iter().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => panic!("Invalid char"),
        }
        if count < 0 {
            return i + 1
        }
    }

    unreachable!();
}