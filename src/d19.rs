use std::collections::{HashMap, HashSet, VecDeque};

const MOLECULE: &str = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";
const TEST_MOLECULE: &str = "HOH";

fn parse(input: &'static str) -> HashMap<&str, Vec<&str>> {
    let mut replacements = HashMap::new();

    for line in input.lines() {
        let parts = line.split("=>").map(|p| p.trim()).collect::<Vec<&str>>();
        let entry = replacements.entry(parts[0]).or_insert(vec![]);
        entry.push(parts[1]);
    }

    replacements
}

fn create_molecules(replacements: &HashMap<&str, Vec<&str>>, m: &str) -> HashSet<String> {
    let mut molecules = HashSet::new();

    for (to_replace, replace_vals) in replacements {
        for val in replace_vals {
            for (i, _) in m.match_indices(to_replace) {
                let mut molecule = String::from(m);
                molecule.replace_range(i..i + to_replace.len(), val);
                molecules.insert(molecule);
            }
        }
    }

    molecules
}

fn replace(replacements: &HashMap<&str, &str>, mut m: String) -> usize {
    let mut steps = 0;
    while &m != "e" {
        for (to_replace, replace_val) in replacements {
            if let Some(i) = m.find(to_replace) {
                m.replace_range(i..i + to_replace.len(), replace_val);
                steps += 1;
            }
        }
    }

    steps
}

pub fn get_solution_1() -> usize {
    let replacements = parse(include_str!("../data/d19.txt"));
    let molecules = create_molecules(&replacements, MOLECULE);
    
    molecules.len()
}

pub fn get_solution_2() -> usize {
    let replacements = parse(include_str!("../data/d19.txt"));
    let flipped = flip_hash_map(replacements);
    replace(&flipped, MOLECULE.to_string())
}

fn flip_hash_map<'a>(map: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, &'a str> {
    let mut flipped = HashMap::new();

    for (k, v) in map {
        for new_k in v {
            assert!(!flipped.contains_key(new_k));
            flipped.insert(new_k, k);
        }
    }

    flipped
}

#[test]
fn test_create_molecules() {
    let replacements = parse(include_str!("../data/d19_test.txt"));
    let molecules = create_molecules(&replacements, TEST_MOLECULE);
    println!("{:?}", molecules);
}

#[test]
fn test_create_medicine() {
    let replacements = parse(include_str!("../data/d19_test.txt"));
    let flipped = flip_hash_map(replacements);
    let steps = replace(&flipped, TEST_MOLECULE.to_string());
    println!("{:?}", steps)
}

#[test]
fn test_flip_hash_map() {
    let replacements = parse(include_str!("../data/d19.txt"));
    let flipped = flip_hash_map(replacements);
    assert_eq!(flipped.len(), 43);
}