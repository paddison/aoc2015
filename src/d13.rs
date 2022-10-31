use std::{collections::HashMap, ops::{Deref, DerefMut}, fmt::Display};

#[derive(Clone)]
struct Graph {
    adj_m: Vec<Vec<isize>>,
}

impl Graph {
    fn add_myself(&mut self) {
        let new_size = self[0].len() + 1;
        for row in &mut self.adj_m {
            row.push(isize::MIN);
        }
        self.adj_m.push(vec![isize::MIN; self.adj_m[0].len()]);
        self.adj_m[new_size - 1][new_size - 1] = 0;
    }
    fn remove_node(&mut self, id: usize) {
        for other_id in 0..self.adj_m.len() {
            self[id][other_id] = 0;
            self[other_id][id] = 0;
        }
    }

    fn remove_edge(&mut self, id: usize, other_id: usize) {
        self[id][other_id] = 0;
        self[other_id][id] = 0;
    }

    fn make_node_unreachable(&mut self, id: usize) {
        for other_id in 0..self.adj_m.len() {
            self[other_id][id] = 0;
        }
    }

    // neighbour_id, cost
    fn get_neighbours(&self, id: usize) -> Vec<(usize, isize)> {
        let mut neighbours = Vec::new();
        
        for (neighbour_id, cost) in self[id].iter().enumerate() {
            if cost != &0 {
                if cost == &isize::MIN {
                    neighbours.push((neighbour_id, 0));
                } else {
                    neighbours.push((neighbour_id, *cost));
                }
            }
        }
        neighbours
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
            let _ = write!(string, "{:?}\n", line);
        }

        write!(f, "{}", string)
    }
}

fn calc_optimal_arrangement(g: Graph) -> isize {
    let mut costs = Vec::new();
    for start_node in 0..g.adj_m.len() {
        for (neighbour, edge_cost) in g.get_neighbours(start_node) {
            if neighbour != start_node {
                let mut g_copy = g.clone();
                let other_edge_cost = g[neighbour][start_node];
                g_copy.remove_edge(start_node, neighbour);
                brute_force_all_costs(start_node, neighbour, g_copy, 0 + edge_cost + other_edge_cost, &mut costs);
            }
        }
    }

    *costs.iter().max().unwrap()
}

fn brute_force_all_costs(start_node: usize, cur_node: usize, g: Graph, cost: isize, costs: &mut Vec<isize>) {
    let edges = g.get_neighbours(cur_node);
    if edges.len() == 1 {
        costs.push(cost + edges[0].1 + g[edges[0].0][cur_node]);
        return;
    }
    for (neighbour, edge_cost) in edges {
        if neighbour != start_node {
            let mut g_copy = g.clone();
            let other_edge_cost = g_copy[neighbour][cur_node];
            g_copy.make_node_unreachable(cur_node);
            brute_force_all_costs(start_node, neighbour, g_copy, cost + edge_cost + other_edge_cost, costs);
        }
    }
}

fn parse(input: &'static str) -> Graph {
    let mut lookup_table = HashMap::new();
    let mut person_id_count = 0;
    let seating_data = input.split('\n')
                            .map(|line| line[..line.len() - 1].split_whitespace().collect::<Vec<&str>>())
                            .map(|split_line| (split_line[0], split_line[10], if split_line[2] == "lose" { split_line[3].parse::<isize>().unwrap() * -1 } else { split_line[3].parse::<isize>().unwrap()}))
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
    calc_optimal_arrangement(g)
}

pub fn get_solution_2() -> isize {
    let mut g = parse(include_str!("../data/d13.txt"));
    g.add_myself();
    calc_optimal_arrangement(g)
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
fn test_calculate_optimal_arrangement() {
    let g = parse(include_str!("../data/d13_test.txt"));
    assert_eq!(calc_optimal_arrangement(g), 330);
}

#[test]
fn test_add_myself() {
    let mut g = parse(include_str!("../data/d13.txt"));
    g.add_myself();
    println!("{}", g);
}