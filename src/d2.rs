fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    input.split('\n')
         .map(|line| line.split('x')
                         .map(|n| n.parse::<usize>().unwrap())
                         .collect::<Vec<usize>>())
         .map(|line| (line[0], line[1], line[2]))
         .collect()
}

pub fn get_solution_1() -> usize {
    let dims = parse(include_str!("../data/d2.txt"));
    let mut square_feet_wrapping_paper = 0;
    for (l, h, w) in dims {
        let l_h = l * h;
        let l_w = l * w;
        let h_w = h * w;
        let min = l_h.min(l_w.min(h_w));
        square_feet_wrapping_paper += 2 * (l_h + l_w + h_w) + min
    }

    square_feet_wrapping_paper
}

pub fn get_solution_2() -> usize {
    let dims = parse(include_str!("../data/d2.txt"));
    let mut ribbon_length = 0;
    for (l, h, w) in dims {
        let wrapping_length = if w >= l && w >= h {
            2 * l + 2 * h
        } else if h >= l && h >= w {
            2 * l + 2 * w
        } else {
            2 * h + 2 * w
        };
        ribbon_length += wrapping_length + l * h * w;
    }

    ribbon_length
}