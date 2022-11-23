use std::{str::Chars, collections::HashMap};

const INVALID_PATS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn parse(input: &str) -> Vec<String> {
    input.split('\n').map(|s| s.to_string()).collect()
}

pub fn get_solution_1() -> usize {
    let mut nice_count = 0;

    for string in parse(include_str!("../data/d5.txt")) {
        if test_string_p1(string) {
            nice_count += 1;
        }
    }

    nice_count
}

pub fn get_solution_2() -> usize {
    let mut nice_count = 0;
    for string in parse(include_str!("../data/d5.txt")) {
        if test_string_p2(string) {
            nice_count += 1;
        }
    }

    nice_count
}

fn test_string_p1(string: String) -> bool {
    for pat in INVALID_PATS {
        if string.contains(pat) {
            return false;
        }
    }
    let mut vowel_count = 0;
    let mut chars = string.chars();

    for c in chars.by_ref() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
        if vowel_count == 3 {
            break; 
        }
    }

    if vowel_count != 3 {
        return false;
    }
    
    let mut chars = string.chars();
    let mut cur = chars.next().unwrap();

    loop {
        if let Some(next) = chars.next() {
            if next == cur {
                return true;
            } else {
                cur = next;
            }

        } else {
            return false;
        }
    }
}

fn test_string_p2(string: String) -> bool {
    contains_double_pair(string.chars()) && repeats_with_between(string.chars())
}

fn contains_double_pair(mut chars: Chars<'_>) -> bool {
    let mut pairs = HashMap::new();
    let mut cur = chars.next().unwrap();
    for (i, next) in chars.enumerate() {
        match pairs.get(&(cur, next)) {
            Some(idx) => if *idx != i - 1 { return true; },
            None => { pairs.insert((cur, next), i); },
        }
        cur = next;
    }
    
    false
}

fn repeats_with_between(mut chars: Chars<'_>) -> bool {
    let mut front = chars.next().unwrap();
    let mut middle = chars.next().unwrap(); 
    for back in chars {
        if front == back {
            return true;
        }
        front = middle;
        middle = back;
    }

    false
}

#[test]
fn test_contains_double_pair() {
    assert!(contains_double_pair("aabcdefgaa".chars()));
    assert!(contains_double_pair("uurcxstgmygtbstg".chars()));
    assert!(!contains_double_pair("aaa".chars()));
}

#[test]
fn test_repeats_with_between() {
    assert!(repeats_with_between("xyx".chars()));
    assert!(repeats_with_between("abcdefeghi".chars()));
    assert!(repeats_with_between("aaa".chars()));
    assert!(!repeats_with_between("uurcxstgmygtbstg".chars()));
    assert!(repeats_with_between("ieodomkazucvgmuy".chars()));

}