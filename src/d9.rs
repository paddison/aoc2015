use std::{collections::{HashMap, BinaryHeap}, fmt::{Display, Write}, ops::{Deref, DerefMut} };

const GRAPH_DIM: usize = 8;

struct Edge(usize, usize, usize); // left, right, cost

#[derive(Debug, Clone)]
struct Graph {
    matrix: [[usize; GRAPH_DIM]; GRAPH_DIM],
    // nodes: HashMap<usize, &'static str>,
}

impl Graph {
    fn get_all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        for (i, row) in self.iter().enumerate() {
            for (j, cost) in row.iter().enumerate() {
                if cost > &0 {
                    edges.push(Edge(i, j, *cost));
                }
            }
        }
        edges
    }

    fn get_neighbours(&self, node: usize) -> Vec<Edge> {
        let mut edges = Vec::new();
        for (j, cost) in self[node].iter().enumerate() {
            if node != j && cost > &0 {
                edges.push(Edge(node, j, *cost));
            }
        }

        edges
    }

    fn remove_node(&mut self, node: usize) {
        for other in 0..GRAPH_DIM {
            self[node][other] = 0;
            self[other][node] = 0;
        }
    }
}

impl Deref for Graph {
    type Target = [[usize; GRAPH_DIM]; GRAPH_DIM];

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for line in &self.matrix {
            let _ = write!(string, "{:?}\n", line);
        }

        write!(f, "{}", string)
    }
}

fn parse(input: &'static str) -> Graph {
    let mut lookup_table = HashMap::new();
    // let mut nodes = HashMap::new();
    let mut matrix = [[0; GRAPH_DIM]; GRAPH_DIM];
    let mut id_count = 0;
    
    let data: Vec<(&str, &str, usize)> = input.split('\n')
                                              .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
                                              .map(|parts| (parts[0], parts[2], parts[4].parse::<usize>().unwrap()))
                                              .collect();
    for (left, right, cost) in data {
        let left_id = *lookup_table.entry(left).or_insert_with(|| { let id = id_count; id_count += 1; id });
        let right_id = *lookup_table.entry(right).or_insert_with(|| { let id = id_count; id_count += 1; id });
        // nodes.insert(left_id, left);
        // nodes.insert(right_id, right);

        matrix[left_id][right_id] = cost;
        matrix[right_id][left_id] = cost; // because the graph is bidirectional
    }

    Graph{ matrix }
}

fn calculate_all_costs(g: Graph) -> Vec<usize>{
    let mut costs = Vec::new();
    for start in 0..GRAPH_DIM {
        for end in 0..GRAPH_DIM {
            if start != end {
                calculate_all_paths(start, end, g.clone(), &mut costs, 0)
            }
        }
    }
    println!("{}", costs.len());
    costs
}

fn calculate_all_paths(start: usize, end: usize, g: Graph, costs: &mut Vec<usize>, cur_cost: usize) {
    let mut neighbours = g.get_neighbours(start);
    if neighbours.len() == 1 {
        let edge = neighbours.pop().unwrap();
        costs.push(cur_cost + edge.2);
        return;
    }

    for next in neighbours {
        if next.1 != end {
            let mut g_next = g.clone();
            g_next.remove_node(next.0);
            calculate_all_paths(next.1, end, g_next, costs, cur_cost + next.2);
        }
    }
}

pub fn get_solution_1() -> usize {
    let g = parse(include_str!("../data/d9.txt"));
    *calculate_all_costs(g).iter().min().unwrap()
}

pub fn get_solution_2() -> usize {
    let g = parse(include_str!("../data/d9.txt"));
    *calculate_all_costs(g).iter().max().unwrap()
}

#[test]
fn test_parse() {
    let g = parse(include_str!("../data/d9.txt"));
    println!("{}", g);
}

#[test]
fn test_get_all_edges() {
    let g = parse(include_str!("../data/d9.txt"));
    let mut last = 0;
    let mut edges = g.get_all_edges();

    while !edges.is_empty() {
        let Edge(_, _, cost) = edges.pop().unwrap();
        assert!(cost >= last);
        last = cost;
    }
}

#[test]
fn test_calculate_all_costs() {
    let g = parse(include_str!("../data/d9.txt"));
    calculate_all_costs(g); 
}