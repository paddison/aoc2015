use std::{collections::HashMap, fmt::{Display, Write}, ops::{Deref, DerefMut}, thread, cmp::Ordering };

const GRAPH_DIM: usize = 8;

const partitions: [(usize, usize); 56] = [
    (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
    (1, 0), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
    (2, 0), (2, 1), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
    (3, 0), (3, 1), (3, 2), (3, 4), (3, 5), (3, 6), (3, 7),
    (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (4, 6), (4, 7),
    (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 6), (5, 7),
    (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 7),
    (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6),
];

#[derive(Debug, Clone)]
struct Graph([[u16; GRAPH_DIM]; GRAPH_DIM]);

impl Graph {
    fn get_neighbours(&self, node: usize) -> Vec<(usize, u16)> {
        let mut neighbours = Vec::new();
        for (j, cost) in self[node].iter().enumerate() {
            if node != j && cost > &0 {
                neighbours.push((j, *cost));
            }
        }

        neighbours
    }

    fn remove_node(&mut self, node: usize) {
        for other in 0..GRAPH_DIM {
            self[node][other] = 0;
            self[other][node] = 0;
        }
    }
}

impl Deref for Graph {
    type Target = [[u16; GRAPH_DIM]; GRAPH_DIM];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for line in &self.0 {
            let _ = write!(string, "{:?}\n", line);
        }

        write!(f, "{}", string)
    }
}

fn parse(input: &'static str) -> Graph {
    let mut lookup_table = HashMap::new();
    let mut matrix = [[0; GRAPH_DIM]; GRAPH_DIM];
    let mut id_count = 0;
    
    let data: Vec<(&str, &str, u16)> = input.split('\n')
                                              .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
                                              .map(|parts| (parts[0], parts[2], parts[4].parse::<u16>().unwrap()))
                                              .collect();
    for (left, right, cost) in data {
        let left_id = *lookup_table.entry(left).or_insert_with(|| { let id = id_count; id_count += 1; id });
        let right_id = *lookup_table.entry(right).or_insert_with(|| { let id = id_count; id_count += 1; id });

        matrix[left_id][right_id] = cost;
        matrix[right_id][left_id] = cost; // because the graph is bidirectional
    }

    Graph(matrix)
}

fn calculate_all_costs(g: Graph) -> Vec<Vec<u16>> {
    let mut handles = Vec::new();
    let mut all_costs = Vec::new();
    
    for p in [&partitions[..28], &partitions[28..]] {
        let g_clone = g.clone();
        let handle = thread::spawn(move || {
            let mut costs = Vec::new();
            for (start, end) in p {
                calculate_all_paths(*start, *end, g_clone.clone(), &mut costs, 0)
            }
            costs
        });

        handles.push(handle);
    }
    for handle in handles {
        all_costs.push(handle.join().unwrap());
    }

    all_costs
}

fn calculate_all_paths(start: usize, end: usize, mut g: Graph, costs: &mut Vec<u16>, cur_cost: u16) {
    let mut neighbours = g.get_neighbours(start);
    g.remove_node(start);

    if neighbours.len() == 1 {
        let (_, cost) = neighbours.pop().unwrap();
        costs.push(cur_cost + cost);
        return;
    }

    for (next, cost) in neighbours {
        if next != end {
            calculate_all_paths(next, end, g.clone(), costs, cur_cost + cost);
        }
    }
}

pub fn get_solution_1() -> u16 {
    let g = parse(include_str!("../data/d9.txt"));
    calculate_all_costs(g).into_iter().flatten().min().unwrap()
}

pub fn get_solution_2() -> u16 {
    let g = parse(include_str!("../data/d9.txt"));
    calculate_all_costs(g).into_iter().flatten().max().unwrap()
}

#[test]
fn test_parse() {
    let g = parse(include_str!("../data/d9.txt"));
    println!("{}", g);
}

#[test]
fn test_calculate_all_costs() {
    let g = parse(include_str!("../data/d9_test.txt"));
    println!("{:?}", calculate_all_costs(g)); 
}