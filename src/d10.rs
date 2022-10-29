const INPUT: &str = "3113322113";

pub fn get_solution_1() -> usize {
    let mut seq = INPUT.to_string();
    for _ in 0..40 {
        seq = widen(seq);
    }

    seq.len()
}

pub fn get_solution_2() -> usize {
    let mut seq = INPUT.to_string();
    for _ in 0..50 {
        seq = widen(seq);
    }

    seq.len()
}

fn widen(seq: String) -> String {
    let mut widened_string = String::new();
    let mut chars = seq.chars();
    let mut cur = chars.next().unwrap();
    let mut count = 1;
    for c in chars {
        if c != cur {
            widened_string.push_str(&count.to_string());
            widened_string.push(cur);
            count = 1;
            cur = c;
        } else {
            count += 1;
        }
    }
    widened_string.push_str(&count.to_string());
    widened_string.push(cur);

    widened_string
}


#[test]
fn test_widen() {
    let mut seq = String::from("1");
    seq = widen(seq);
    assert_eq!(seq, "11".to_string());
    
    seq = widen(seq);
    assert_eq!(seq, "21".to_string());

    seq = widen(seq);
    assert_eq!(seq, "1211".to_string());

    seq = widen(seq);
    assert_eq!(seq, "111221".to_string());

    seq = widen(seq);
    assert_eq!(seq, "312211".to_string());
    

}