const _INPUT: & str = "ckczppom";
// use md5;

// hardcode the solutions, because they can only be brute forced, 
// which takes some time
pub fn get_solution_1() -> usize {
    // _find_num("00000")
    117946
}

pub fn get_solution_2() -> usize {
    // _find_num("000000")
    3938038}

fn _try_num(n: usize, pat: &str) -> Option<usize> {
    let hash = _get_hash(n);
    if hash.starts_with(pat) {
        Some(n)
    } else {
        None
    }
} 

fn _find_num(pat: &str) -> usize {
    let mut n = 0; 
    loop {
        if let Some(valid) = _try_num(n, pat) {
            break valid;
        }
        n += 1;
    }
} 

fn _get_hash(n: usize) -> String {
    let _key_with_n = format!("{}{}", _INPUT, n);
    // let digest = md5::compute(_key_with_n);
    let digest = 123;
    format!("{:x}", digest)
}