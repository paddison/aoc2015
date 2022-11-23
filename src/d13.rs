use std::{collections::HashMap, ops::{Deref, DerefMut}, fmt::Display};

#[derive(Clone)]
struct Graph {
    adj_m: Vec<Vec<isize>>,
}

impl Graph {
    fn add_myself(&mut self) {
        let new_size = self[0].len() + 1;
        for row in &mut self.adj_m {
            row.push(0);
        }
        self.adj_m.push(vec![0; self.adj_m[0].len()]);
        self.adj_m[new_size - 1][new_size - 1] = 0;
    }
}

impl Deref for Graph {
    type Target = Vec<Vec<isize>>;

    fn deref(&self) -> &Self::Target {
        &self.adj_m
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.adj_m
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let mut string = String::new();
        for line in &self.adj_m {
            let _ = writeln!(string, "{:?}", line);
        }

        write!(f, "{}", string)
    }
}

fn parse(input: &'static str) -> Graph {
    let mut lookup_table = HashMap::new();
    let mut person_id_count = 0;
    let seating_data = input.split('\n')
                            .map(|line| line[..line.len() - 1].split_whitespace().collect::<Vec<&str>>())
                            .map(|split_line| (split_line[0], split_line[10], if split_line[2] == "lose" { -split_line[3].parse::<isize>().unwrap() } else { split_line[3].parse::<isize>().unwrap()}))
                            .collect::<Vec<(&str, &str, isize)>>();

    let mut adj_m = if seating_data.len() == 56 {
        vec![vec![0; 8]; 8]
    } else {
        vec![vec![0; 4]; 4]
    };

    let mut clj = || {
        let cur_id = person_id_count; 
            person_id_count += 1;
            cur_id
    };
    
    for (person, other, cost) in seating_data {
        let person_id = *lookup_table.entry(person).or_insert_with(&mut clj);
        let other_id = *lookup_table.entry(other).or_insert_with(&mut clj);
        adj_m[person_id][other_id] = cost;
    }

    Graph { adj_m }
}

pub fn get_solution_1() -> isize {
    let g = parse(include_str!("../data/d13.txt"));
    calculate_costs(&g)
}

pub fn get_solution_2() -> isize {
    let mut g = parse(include_str!("../data/d13.txt"));
    g.add_myself();
    calculate_costs(&g)
}

fn get_all_circles(g: &Graph) -> Vec<Vec<usize>> {
    // it shouldn't matter where we start, since we just want to go in a circle
    let n_persons = g.len();
    let mut all_perms = Vec::new();
    let mut initial = Vec::new();
    for i in 1..n_persons {
        initial.push(i);
    }
    
    heap_permutations(initial.len(), &mut initial, &mut all_perms);
    for perm in all_perms.iter_mut() {
        perm.insert(0, 0);
    }
    all_perms
}

// same implementation as https://www.geeksforgeeks.org/heaps-algorithm-for-generating-permutations
fn heap_permutations(k: usize, perm: &mut Vec<usize>, all_perms: &mut Vec<Vec<usize>>) {
    if k == 1 {
        all_perms.push(perm.clone())
    } else {
        for i in 0..k {
            heap_permutations(k - 1, perm, all_perms);

            if k % 2 == 0 {
                perm.swap(i, k - 1);
            } else {
                perm.swap(0, k - 1);
            }
        }
    }
}

fn calculate_costs(g: &Graph) -> isize {
    let perms = get_all_circles(g);
    let mut costs = Vec::new();
    let n_persons = g.adj_m.len();
    for perm in perms {
        let mut cost = 0;
        for (i, cur_person) in perm.iter().enumerate() {
            let neighbour = perm[(i + 1) % n_persons];
            cost += g[neighbour][*cur_person];
            cost += g[*cur_person][neighbour];
        }
        costs.push(cost);
    }

    *costs.iter().max().unwrap()
}

#[test]
fn test_parse() {
    let g = parse(include_str!("../data/d13_test.txt"));
    let expected = vec![
        vec![0, 54, -79, -2],
        vec![83, 0, -7, -63],
        vec![-62, 60, 0, 55],
        vec![46, -7, 41, 0],
    ];

    assert_eq!(g.adj_m, expected);
}

#[test]
fn test_get_all_perms() {
    let g = parse(include_str!("../data/d13_test.txt"));
    println!("{:?}", get_all_circles(&g));
}

#[test]
fn test_calc_all_costs_with_permutation() {
    let g = parse(include_str!("../data/d13_test.txt"));
    assert_eq!(330, calculate_costs(&g));
}