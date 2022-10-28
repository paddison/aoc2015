use std::collections::HashMap;

// rewrite so that wires are stored in hashmap
// key is wirename, value is gate
// store cache which keeps calculated wires
// check cache for value,
// if not found recursively calculate the value

macro_rules! calculate_gate {
    ($lhs:expr, $rhs:expr, $cache:expr, $gates:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Input::Wire(lhs), Input::Wire(rhs)) => calculate_wire(lhs, $gates.get(lhs).unwrap(), $cache, $gates) $op calculate_wire(rhs, $gates.get(rhs).unwrap(), $cache, $gates),
            (Input::Wire(lhs), Input::Const(rhs)) => calculate_wire(lhs, $gates.get(lhs).unwrap(), $cache, $gates) $op *rhs,
            (Input::Const(lhs), Input::Wire(rhs)) => *lhs $op calculate_wire(rhs, $gates.get(rhs).unwrap(), $cache, $gates),
            (Input::Const(lhs), Input::Const(rhs)) => *lhs $op *rhs,
        }
    }
}

fn calculate_circuit(gates: HashMap<&'static str, Gate>) -> HashMap<&'static str, u16> {
    let mut cache = HashMap::new();

    for (wire, gate) in &gates {
        let _ = calculate_wire(wire, &gate, &mut cache, &gates);
    }
    
    cache
}

// called recursively
fn calculate_wire(wire: &'static str, gate: &Gate, cache: &mut HashMap<&'static str, u16>, gates: &HashMap<&'static str, Gate>) -> u16 {
    if let Some(val) = cache.get(wire) {
        return *val;
    } 
    let v = match gate {
        Gate::Assign(inp) => match inp {
            Input::Wire(n) => calculate_wire(n, gates.get(n).unwrap(), cache, gates),
            Input::Const(v) => *v,
        },
        Gate::Not(inp) => match inp {
            Input::Wire(n) => !calculate_wire(n, gates.get(n).unwrap(), cache, gates),
            Input::Const(v) => !*v,
        },
        Gate::And(lhs, rhs) => calculate_gate!(lhs, rhs, cache, gates, &),
        Gate::Or(lhs, rhs) => calculate_gate!(lhs, rhs, cache, gates, |),
        Gate::Rshift(lhs, rhs) => calculate_gate!(lhs, rhs, cache, gates, >>),
        Gate::Lshift(lhs, rhs) => calculate_gate!(lhs, rhs, cache, gates, <<),
    };

    cache.insert(wire, v);
    v
}

#[derive(Debug)]
enum Input {
    Wire(&'static str),
    Const(u16),
}

impl From<&'static str> for Input {
    fn from(val: &'static str) -> Self {
        match val.parse::<u16>() {
            Ok(n) => Input::Const(n),
            Err(_) => Input::Wire(val),
        }
    }
}

#[derive(Debug)]
enum Gate {
    Assign(Input), // wire const
    Not(Input), // wire
    And(Input, Input), // wire const
    Or(Input, Input), //
    Rshift(Input, Input), // wire const
    Lshift(Input, Input), // wire const
}

impl From<Vec<&'static str>> for Gate {
    fn from(gate_parts: Vec<&'static str>) -> Self {
        if gate_parts.len() == 2 {
            Gate::Assign(gate_parts[0].into())
        } else if gate_parts.len() == 3 {
            Gate::Not(gate_parts[1].into())
        } else {
            let (lhs, rhs) = (Input::from(gate_parts[0]), Input::from(gate_parts[2]));
            match gate_parts[1] {
                "AND" => Gate::And(lhs, rhs),
                "OR" => Gate::Or(lhs, rhs),
                "RSHIFT" => Gate::Rshift(lhs, rhs),
                "LSHIFT" => Gate::Lshift(lhs, rhs),
                _ => panic!("Unknown instruction")
            }
        }
    }
}

fn parse(input: &'static str) -> HashMap<&'static str, Gate> {
    let mut instructions = HashMap::new();
    for line in input.split('\n') {
        let gate_parts = line.split("->")
                        .flat_map(|p| p.split_whitespace().collect::<Vec<&str>>())
                        .collect::<Vec<&str>>();
        
        instructions.insert(*gate_parts.last().unwrap(), Gate::from(gate_parts));
    }   

    instructions
}

pub fn get_solution_1() -> u16 {
    let gates = parse(include_str!("../data/d7.txt"));

    let calculated_circuit = calculate_circuit(gates);
    *calculated_circuit.get("a").unwrap()
}

pub fn get_solution_2() -> u16 {
    let mut gates = parse(include_str!("../data/d7.txt"));
    let b_wire = gates.get_mut("b").unwrap();
    *b_wire = Gate::Assign(Input::Const(46065));
    let calculated_circuit = calculate_circuit(gates);
    *calculated_circuit.get("a").unwrap()
}

#[test]
fn test_parse_circuit() {
    let gates = parse(
"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i");

    println!("{:?}", gates);
}

#[test]
fn test_calculate_circuit() {
    let gates = parse(
        "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i");
    let calculated_circuit = calculate_circuit(gates);
    println!("{:?}", calculated_circuit);
}