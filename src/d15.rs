// Returns coefficients as iterator, much slower than hardcoding loop
struct _CoeffIter {
    dim: usize,
    coeffs: Vec<usize>,
    max: usize,
}

impl _CoeffIter {
    fn _new(dim: usize, max: usize) -> Self {
        let mut coeffs = vec![0; dim];
        coeffs[dim - 1] = max;
        Self { dim, coeffs, max }
    }
}

impl Iterator for _CoeffIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coeffs[0] == 100 {
            return None;
        }
        let next = self.coeffs.clone();
        let mut flips = true;
        let mut idx = self.dim - 2; // start at second to last element
        while flips {
            flips = false;
            self.coeffs[idx] = (self.coeffs[idx] + 1) % (self.max - self.coeffs[..idx].iter().sum::<usize>() + 1);
            self.coeffs[self.dim - 1] = self.max - self.coeffs[..self.dim - 1].iter().sum::<usize>();
            if self.coeffs[idx] == 0 && idx != 0 {
                flips = true;
                idx -= 1;
            }
        }

        Some(next)
    }
}

#[derive(Clone, Copy)]
enum Prop {
    Capacity,
    Durability,
    Flavor,
    Texture,
    Calories,
}

type Ingr = [isize; 5]; // cap, dur, flav, text, cal

fn parse(input: &'static str) -> Vec<Ingr> {
    input.split('\n')
         .map(|line| line.split_whitespace().collect::<Vec<&str>>())
         .map(|parts| [
            parts[2].trim_end_matches(',').parse::<isize>().unwrap(),
            parts[4].trim_end_matches(',').parse::<isize>().unwrap(),
            parts[6].trim_end_matches(',').parse::<isize>().unwrap(),
            parts[8].trim_end_matches(',').parse::<isize>().unwrap(),
            parts[10].trim_end_matches(',').parse::<isize>().unwrap()])
         .collect()
}

pub fn get_solution_1() -> isize {
    let ingr = parse(include_str!("../data/d15.txt"));
    let mut max = 0;

    for x in 0..100 {
        for y in 0..(100 - x) {
            for z in 0..(100 - x - y) {
                let u = 100 - x - y - z;
                let coeff = [x, y, z, u];
                let (capacity, durability, flavor, texture, _) = calculate_all(coeff.as_slice(), &ingr);

                if capacity > 0 && durability > 0 && flavor > 0 && texture > 0 {
                    max = std::cmp::max(max, capacity * durability * flavor * texture);
                }
            }
        }
    }

    max
}

fn calculate_prop(coeff: &[usize], ingredients: &[Ingr], prop: Prop) -> isize {
    ingredients.iter().enumerate().map(|(i, ingr)| ingr[prop as usize] * coeff[i] as isize).sum::<isize>()
}

fn calculate_all(coeff: &[usize], ingr: &[Ingr]) -> (isize, isize, isize, isize, isize) {
    let capacity = calculate_prop(coeff, ingr, Prop::Capacity);
    let durability = calculate_prop(coeff, ingr, Prop::Durability);
    let flavor = calculate_prop(coeff, ingr, Prop::Flavor);
    let texture = calculate_prop(coeff, ingr, Prop::Texture);
    let calories = calculate_prop(coeff, ingr, Prop::Calories);

    (capacity, durability, flavor, texture, calories)
}

pub fn get_solution_2() -> isize {
    let ingr = parse(include_str!("../data/d15.txt"));
    let mut coeff = vec![0; 4];
    let mut vals = Vec::new();
    
    solve_recur(&mut coeff, &ingr, 0, &mut vals);

    *vals.iter().max().unwrap()
}

fn solve_recur(coeff: &mut [usize], ingr: &[Ingr], loop_depth: usize, vals: &mut Vec<isize>) {
    if ingr.len() - 1 == loop_depth {
        coeff[loop_depth] = 100 - coeff[..loop_depth].iter().sum::<usize>();

        let (capacity, durability, flavor, texture, calories) = calculate_all(coeff, ingr);

        if capacity > 0 && durability > 0 && flavor > 0 && texture > 0 && calories == 500 {
            vals.push(capacity * durability * flavor * texture);
        }
    } else {
        let range_max: isize = coeff[..loop_depth].iter().sum::<usize>() as isize;

        for c in 0..100 - range_max {
            coeff[loop_depth] = c as usize;
            solve_recur(coeff, ingr, loop_depth + 1, vals)
        }
    }
}