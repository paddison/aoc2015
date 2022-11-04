fn parse(input: &'static str) -> Vec<u16> {
    input.split('\n').map(|n| n.parse::<u16>().unwrap()).collect()
}

fn count_combinations(containers: &[u16]) -> usize {
    let mut count = 0;

    for mut i in 0..(1 << containers.len()) {
        let mut sum = 0;
        for item in containers {
            if i % 2 == 1 {
                sum += item;
            }
            i >>= 1;
        }
        if sum == 150 {
            count += 1;
        }
    }

    count
}

fn count_containers(containers: &[u16]) -> Vec<u8> {
    let mut n_containers = Vec::new();

    for mut i in 0..(1 << containers.len()) {
        let mut cont_count = 0;
        let mut sum = 0;
        for item in containers {
            if i % 2 == 1 {
                cont_count += 1;
                sum += item;
            }
            i >>= 1;
        }
        if sum == 150 {
            n_containers.push(cont_count);
        }
    }

    n_containers
}

pub fn get_solution_1() -> usize {
    let containers = parse(include_str!("../data/d17.txt"));
    count_combinations(&containers)
}

pub fn get_solution_2() -> usize {
    let containers = parse(include_str!("../data/d17.txt"));
    let ps = count_containers(&containers);
    let min = *ps.iter().min().unwrap();
    ps.into_iter().filter(|n_cont| n_cont == &min).count()
}