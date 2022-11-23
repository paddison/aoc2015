use std::collections::{HashSet, HashMap};

const _INPUT: usize = 33100000;

pub fn get_solution_1() -> usize {
    // calculate_house_number()
    776160
}

pub fn get_solution_2() -> usize {
    // calculate_house_number_cached()
    786240
}

fn _calculate_presents(house_number: usize) -> usize {
    _get_factors(house_number)
        .iter()
        .fold(0, |acc, factor| acc + factor * 10)
}

fn _calculate_presents_cached(house_number: usize, elves: &mut HashMap<usize, usize>) -> usize {
    _get_factors_cached(house_number, elves)
        .iter()
        .fold(0, |acc, factor| acc + factor * 11)
}

fn _get_factors_cached(n: usize, cache: &mut HashMap<usize, usize>) -> HashSet<usize> {
    let mut factors = HashSet::from([1, n]);
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
    }
    factors.into_iter().filter(|fac| {
        let entry = cache.entry(*fac).or_insert(0);
        *entry += 1;
        *entry <= 50
    }).collect::<HashSet<usize>>()
}

fn _get_factors(n: usize) -> HashSet<usize> {
    let mut factors = HashSet::from([1, n]);
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
    }

    factors
}

fn _calculate_house_number() -> usize {
    for house_number in (_INPUT / 1000).. {
        let p = _calculate_presents(house_number);
        // println!("{}", p);
        if p >= _INPUT {
            return house_number;
        }
    }

    unreachable!();
}

fn _calculate_house_number_cached() -> usize {
    let mut elves = HashMap::new();
    for house_number in 1.. {
        let p = _calculate_presents_cached(house_number, &mut elves);
        if p >= _INPUT {
            return house_number;
        }
    }

    unreachable!();
}

#[test]
fn test_calculate_presents() {
    assert_eq!(_calculate_presents(1), 10);
    assert_eq!(_calculate_presents(2), 30);
    assert_eq!(_calculate_presents(3), 40);
    assert_eq!(_calculate_presents(4), 70);
    assert_eq!(_calculate_presents(5), 60);
    assert_eq!(_calculate_presents(6), 120);
    assert_eq!(_calculate_presents(7), 80);
    assert_eq!(_calculate_presents(8), 150);
    assert_eq!(_calculate_presents(9), 130);
}   