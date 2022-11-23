fn parse(input: &'static str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn build_all_sums(required_sum: usize, cur_sum: usize, packages: &[usize], group: Vec<usize>, max_packages: usize) -> Option<Vec<usize>> {
    if group.len() >= max_packages {
        return None;
    }
    for (i, package) in packages.iter().enumerate() {
        let mut group_clone = group.clone();
        if package + cur_sum < required_sum {
            group_clone.push(*package);
            let min_group = build_all_sums(required_sum, cur_sum + package, &packages[i + 1..], group_clone, max_packages);
            if min_group.is_some() {
                return min_group;
            } 
        } else if package + cur_sum == required_sum {
            group_clone.push(*package);
            return Some(group_clone);
        }
    }
    None
}

// try to build the smallest group size from the smallest numbers, in order to get the smallest qe
fn determine_min_qe(n_groups: usize) -> usize {
    let packages = parse(include_str!("../data/d24.txt"));
    let required_sum = packages.iter().sum::<usize>() / n_groups;
    let mut max_packages = 1;
    loop {
        if let Some(group) = build_all_sums(required_sum, 0, &packages, Vec::new(), max_packages) {
            return group.iter().product::<usize>();
        }
        max_packages += 1;
    }
}

pub fn get_solution_1() -> usize {
    determine_min_qe(3)
}

pub fn get_solution_2() -> usize {
    determine_min_qe(4)
}