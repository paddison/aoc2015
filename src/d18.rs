#[derive(Debug)]
struct Grid {
    _inner: Vec<bool>,
    dim: (usize, usize) // (rows, cols)
}

impl Grid {

    #[inline(always)]
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.dim.1 + col
    }

    fn get(&self, row: usize, col: usize) -> Option<&bool>{
        if row >= self.dim.0 || col >= self.dim.1 {
            return None;
        } 

        self._inner.get(self.idx(row, col))
    }

    fn get_on_neighbours(&self, row: usize, col: usize) -> usize {
        let mut n_neighbours = 0;
        let (min_row, max_row) = if row > 0 { (row - 1, row + 2) } else { (0, 2) };
        let (min_col, max_col) = if col > 0 { (col - 1, col + 2) } else { (0, 2) };

        for r in min_row..max_row {
            for c in min_col..max_col {
                if r != row || c != col  {
                    if let Some(&true) = self.get(r, c) {
                        n_neighbours += 1;
                    }   
                }
            }
        }
        
        n_neighbours
    }

    fn stays_on(&self, row: usize, col: usize) -> bool {
        self.get_on_neighbours(row, col) == 2 || self.get_on_neighbours(row, col) == 3
    }

    fn stays_on_p2(&self, row: usize, col: usize) -> bool {
        if  row == 0 && col == 0 || 
            row == 0 && col == self.dim.1 - 1 || 
            row == self.dim.0 - 1 && col == 0 || 
            row == self.dim.0 - 1 && col == self.dim.1 - 1 {
            return true
        } 
        self.get_on_neighbours(row, col) == 2 || self.get_on_neighbours(row, col) == 3
    }

    fn turns_on(&self, row: usize, col: usize) -> bool {
        self.get_on_neighbours(row, col) == 3
    }

    fn count_on(&self) -> usize {
        self._inner.iter().filter(|light| **light).count()
    }

    fn turn_on_edges(&mut self) {
        let tl = self.idx(0, 0);
        let tr = self.idx(0, self.dim.1 - 1);
        let bl = self.idx(self.dim.0 - 1, 0);
        let br = self.idx(self.dim.0 - 1, self.dim.1 - 1);

        self._inner[tl] = true;
        self._inner[tr] = true;
        self._inner[bl] = true;
        self._inner[br] = true;
    }

    fn update<F>(self, stays_on: &F) -> Self 
    where F: Fn(&Grid, usize, usize) -> bool
    {
        let mut new_inner = vec![];
        for row in 0..self.dim.0 {
            for col in 0..self.dim.1 {
                new_inner.push(match *self.get(row, col).unwrap() {
                    true  => stays_on(&self, row, col),
                    false => self.turns_on(row, col),
                })
            }
        }

        Grid { _inner: new_inner, dim: self.dim }
    }
}

fn parse(input: &'static str) -> Grid {
    let dim = input.find('\n').unwrap();
    let _inner = input.lines()
         .flat_map(|l| l.chars())
         .map(|c| c == '#')
         .collect::<Vec<bool>>();

    Grid { _inner, dim: (dim, dim) }
}

fn update_n_times<F>(mut grid: Grid, n: usize, stays_on: F) -> Grid 
where F: Fn(&Grid, usize, usize) -> bool 
{
    for _ in 0..n {
        grid = grid.update(&stays_on);
    }

    grid
}

pub fn get_solution_1() -> usize {
    let mut grid = parse(include_str!("../data/d18.txt"));
    grid = update_n_times(grid, 100, Grid::stays_on);
    grid.count_on()
}

pub fn get_solution_2() -> usize {
    let mut grid = parse(include_str!("../data/d18.txt"));
    grid.turn_on_edges();
    grid = update_n_times(grid, 100, Grid::stays_on_p2);
    grid.count_on()
}

#[test]
fn test_solution() {
    let mut grid = parse(include_str!("../data/d18_test.txt"));
    grid = update_n_times(grid, 4, Grid::stays_on);
    assert_eq!(grid.count_on(), 4);
}

#[test]
fn test_get() {
    let grid = parse(include_str!("../data/d18_test.txt"));
    assert!(!grid.get(0, 0).unwrap());
    assert!(grid.get(1, 5).unwrap());
}

#[test]
fn test_get_on_neighbours() {
    let grid = parse(include_str!("../data/d18_test.txt"));
    assert_eq!(grid.get_on_neighbours(0, 1), 0);
    assert_eq!(grid.get_on_neighbours(0, 2), 3);
    assert_eq!(grid.get_on_neighbours(4, 1), 6);

}

#[test]
fn test_stays_on() {
    let grid = parse(include_str!("../data/d18_test.txt"));
    assert!(grid.stays_on(0, 3));
    assert!(!grid.stays_on(3, 2));
}