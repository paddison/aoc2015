const INPUT: &str = "vzbxkghb";
const ASCII_A: u8 = 97;
const INV_LETTERS: [char; 3] = ['i', 'l', 'o'];

fn increment_pw(pw: &mut str) {
    let chars = unsafe { pw.as_bytes_mut() };
    for c in chars.iter_mut().rev() {
        *c = (*c - (ASCII_A - 1)) % 26 + ASCII_A;
        if c != &ASCII_A {
            break;
        }
    }
}

fn is_valid_pw(pw: &str) -> bool{
    contains_valid_letters(pw) && contains_increasing_letters(pw) && contains_two_pairs(pw)
}

fn contains_valid_letters(pw: &str) -> bool {
    !pw.contains(INV_LETTERS)
}

fn contains_increasing_letters(pw: &str) -> bool {
    let bytes = pw.as_bytes();
    for (i, c) in bytes[..pw.len() - 2].iter().enumerate() {
        if *c + 1 == bytes[i + 1] && *c + 2 == bytes[i + 2] {
            return true
        }
    }

    false
}

fn contains_two_pairs(pw: &str) -> bool {
    let mut pairs = Vec::new();

    for pair in pw.as_bytes().windows(2) {
        if pair[0] == pair[1] && !pairs.contains(&pair[0]){
            pairs.push(pair[0]);
        }
    }

    pairs.len() > 1
}

pub fn get_valid_pw(pw: &mut str) {
    while !is_valid_pw(pw) {
        increment_pw(pw);
    }
}

pub fn get_solution_1() -> String {
    let mut pw = String::from(INPUT);

    get_valid_pw(&mut pw);

    pw
}

pub fn get_solution_2() -> String {
    let mut pw = String::from("vzbxxyzz");
    increment_pw(&mut pw);
    get_valid_pw(&mut pw);
    pw
}

#[test]
fn test_increment_pw() {
    let mut pw = String::from("abz");
    increment_pw(&mut pw);
    assert_eq!(pw, "aca".to_string());
    increment_pw(&mut pw);
    assert_eq!(pw, "acb".to_string());
}

#[test]
fn test_contains_valid_letters() {
    let valid = "abbceffg".to_string();
    assert!(contains_valid_letters(&valid));

    let invalid = "hijklmmn".to_string();
    assert!(!contains_valid_letters(&invalid));
}

#[test]
fn test_contains_increasing_letters() {
    let valid = "hijklmmn".to_string();
    assert!(contains_increasing_letters(&valid));
    
    let invalid = "abbceffg".to_string();
    assert!(!contains_increasing_letters(&invalid));
}  

#[test]
fn test_contains_two_pairs() {
    let valid = "abcdffaa".to_string();
    assert!(contains_two_pairs(&valid));

    let invalid = "abbcegjk".to_string();
    assert!(!contains_two_pairs(&invalid));
}

#[test]
fn find_valid_pw() {
    let mut pw = String::from("abcdefgh");

    while !is_valid_pw(&pw) {
        increment_pw(&mut pw);
    }

    assert_eq!(&pw, "abcdffaa");
}