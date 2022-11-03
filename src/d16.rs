use std::collections::HashMap;

const ACTUAL_SUE: &str =
"children 3
cats 7
samoyeds 2
pomeranians 3
akitas 0
vizslas 0
goldfish 5
trees 3
cars 2
perfumes 1";

type Sue = HashMap<&'static str, usize>;

fn parse(input: &'static str) -> Vec<Sue> {
    let mut sues = Vec::new();
    for parts in input.split('\n').map(|l| l.split_whitespace().collect::<Vec<&'static str>>()) {
        let mut sue = HashMap::new();
        for i in (2..7).step_by(2) {
            sue.insert(parts[i].trim_end_matches(':'), parts[i + 1].trim_end_matches(',').parse::<usize>().unwrap());

        }
        sues.push(sue)
    }

    sues
}

fn get_actual_sue_data() -> Sue {
    let mut sue = HashMap::new();
    ACTUAL_SUE.split('\n')
              .map(|line| line.split_whitespace().collect::<Vec<&str>>())
              .for_each(|parts| { sue.insert(parts[0], parts[1].parse::<usize>().unwrap()); });
   
    sue
}

fn filter_sues<F: FnMut(&(usize, &Sue)) -> bool>(sues: Vec<Sue>, clj: F) -> Option<usize> {
    let result = sues.iter()
                     .enumerate()
                     .filter(clj)
                     .map(|(i, _)| i + 1)
                     .collect::<Vec<usize>>();
    
    result.get(0).cloned()
}



pub fn get_solution_1() -> usize {
    let sues = parse(include_str!("../data/d16.txt"));
    let actual_sue = get_actual_sue_data();
    let clj = |(_, sue): &(usize, &Sue)| {
        for (k, v) in sue.iter() {
            if actual_sue.get(k).unwrap() != v {
                return false;
            }
        }
        true
    };
    
    filter_sues(sues, clj).unwrap_or(0)
}

pub fn get_solution_2() -> usize {
    let sues = parse(include_str!("../data/d16.txt"));
    let actual_sue = get_actual_sue_data();
    let clj = |(_, sue): &(usize, &Sue)| {
        for (k, v) in sue.iter() {
            match k {
                &"cats" | &"trees"              => if actual_sue.get(k).unwrap() >= v { return false; },
                &"pomeranians" | &"goldfish"    => if actual_sue.get(k).unwrap() <= v { return false; },
                _                               => if actual_sue.get(k).unwrap() != v { return false; }
            }
        }
        true
    };
    
    filter_sues(sues, clj).unwrap_or(0)
}