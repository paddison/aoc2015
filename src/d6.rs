use std::{ops::{Deref, DerefMut}, fmt::Display};

const N_WIDTH: usize = std::mem::size_of::<u64>() * 8;

fn parse(input: &str) -> Vec<Instr> {
    let mut instructions = Vec::new();

    for line in input.split('\n') {
        let dims = line.split_ascii_whitespace().flat_map(|l| l.split(',')).filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<usize>>();
        let range = ((dims[0], dims[2]), (dims[1], dims[3]));
        let op = match &line[..7] {
            "turn on" => Op::On,
            "turn of" => Op::Off, 
            "toggle " => Op::Toggle,
            _ => panic!("Invalid instruction type")
        };
        instructions.push((op, range));
    }

    instructions
}

pub fn get_solution_1() -> usize {
    let all_instr = parse(include_str!("../data/d6.txt"));
    let mut g = BinaryGrid::new();
    assert_eq!(all_instr.len(), 300);
    g.handle_all_instr(all_instr);
    g.count_lights()
}

pub fn get_solution_2() -> isize {
    let all_instr = parse(include_str!("../data/d6.txt"));
    let mut g = Grid::new();
    for instr in all_instr {
        g.handle_instr(instr);
    }

    g.count_lights()
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    On,
    Off,
    Toggle,
}

type Instr = (Op, ((usize, usize), (usize, usize)));

#[derive(Debug)]
struct BinaryGrid {
    lights: [u64; 1024 * 1000 / N_WIDTH], // 1600
    dim: (usize, usize), // (x, y)
}

impl Deref for BinaryGrid {
    type Target = [u64; 1024 * 1000 / N_WIDTH];

    fn deref(&self) -> &Self::Target {
        &self.lights
    }
}

impl DerefMut for BinaryGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lights
    }
}

impl Display for BinaryGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (i, n) in self.lights.iter().enumerate() {
            string += &format!("{:b}", n);
            if i % 16 == 0 {
                string += "\n";
            }
        }

        write!(f, "{}", string)
    }
}

impl BinaryGrid {
    fn new() -> Self {
        BinaryGrid { lights: [0; 16000], dim: (1024, 1000) }
    }

    fn count_lights(&self) -> usize {
        let mut count = 0;
        for mut light_num in self.lights {
            if light_num == u64::MAX {
                count += 64;
            } else {
                while light_num > 0 {
                    if light_num % 2 == 1 {
                        count += 1;
                    }
                    light_num /= 2;
                }
            }
        }  
        count 
    }

    fn handle_all_instr(&mut self, all_instr: Vec<Instr>) {
        for instr in all_instr {
            self.handle_instr(instr);
        }
    }

    #[inline(always)]
    fn calc_start_index(&self, x: usize, y: usize) -> usize {
        self.dim.0 / N_WIDTH * y + x / N_WIDTH
    }

    #[inline(always)]
    fn calc_offset(&self, x: usize) -> usize {
        x % N_WIDTH
    }

    fn handle_instr(&mut self, instr: Instr) {
        let (op, ((x0, x1), (_y0, _y1))) = instr;
        let nums = self.calc_numbers(x0, x1);
        
        match op {
            Op::On => self.do_on(nums, instr.1),
            Op::Off => self.do_off(nums, instr.1),
            Op::Toggle => self.do_toggle(nums, instr.1),
        } 
    }

    fn do_on(&mut self, nums: Vec<u64>, ((x0, _x1), (y0, y1)): ((usize, usize), (usize, usize))) {
        // calculate index where to start 
        let start_idx = self.calc_start_index(x0, y0);
        for y in (0..(y0.abs_diff(y1 + 1)) * 16).step_by(16) {
            for (i, n) in nums.iter().enumerate() {
                self[start_idx + i + y] |= n;
            }
        }
    }

    fn do_off(&mut self, mut nums: Vec<u64>, ((x0, _x1), (y0, y1)): ((usize, usize), (usize, usize))) {
        // invert numbers 
        for n in &mut nums {
            *n ^= u64::MAX;
        }
        let start_idx = self.calc_start_index(x0, y0);
        for y in (0..(y0.abs_diff(y1 + 1)) * 16).step_by(16) {
            for (i, n) in nums.iter().enumerate() {
                self[start_idx + i + y] &= n;
            }
        }
    }

    fn do_toggle(&mut self, nums: Vec<u64>, ((x0, _x1), (y0, y1)): ((usize, usize), (usize, usize))) {
        let start_idx = self.calc_start_index(x0, y0);
        for y in (0..(y0.abs_diff(y1 + 1)) * 16).step_by(16) {
            for (i, n) in nums.iter().enumerate() {
                self[start_idx + i + y] ^= n;
            }
        }
    }

    // toggle is xor with ones, 
    // on is or with ones
    // off is and with ones and zeroes
    // number is always length of range in x dir 
    // repeat process for each row
    // numbers are always 64 bit long
    // calculate offset x where number needs to start
    // 
    fn calc_numbers(&self, x0: usize, x1: usize) -> Vec<u64> {
        let mut nums = Vec::new();
        let offset = self.calc_offset(x0);
        let mut width = x0.abs_diff(x1 + 1);

        if offset + width <= N_WIDTH {
            return if width == 64 {
                vec![u64::MAX]
            } else {
                vec![((2_u64.pow(width as u32) - 1)) << (N_WIDTH - width) >> offset]
            }
        }

        let n_width = N_WIDTH - offset;
        let first = if n_width == 64 { u64::MAX } else { 2_u64.pow(n_width as u32) - 1 };
        nums.push(first);

        width -= n_width;

        while width >= N_WIDTH {
            nums.push(u64::MAX);
            width -= N_WIDTH;
        }

        if width > 0 {
            nums.push((2_u64.pow(width as u32) - 1) << (N_WIDTH - width));
        }

        nums
    }
}

struct Grid {
    vals: Vec<isize>,
    dim: (usize, usize) // x, y
}

impl Deref for Grid {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vals
    }
}

impl Grid {
    fn new() -> Self {
        Grid { vals: vec![0; 1000000], dim: (1000, 1000) }
    }

    fn update(&mut self, x: usize, y: usize, val: isize) {
        let idx = y * self.dim.0 + x;
        self[idx] += val;
        if self[idx] < 0 {
            self[idx] = 0;
        }
    }

    fn handle_instr(&mut self, instr: Instr) {
        let ((x0, x1), (y0, y1)) = instr.1;
        match instr.0 {
            Op::On => self.do_on(x0, x1, y0, y1),
            Op::Off => self.do_off(x0, x1, y0, y1),
            Op::Toggle => self.do_toggle(x0, x1, y0, y1),
        }
    }

    fn do_on(&mut self, x0: usize, x1: usize, y0: usize, y1: usize) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.update(x, y, 1)
            }
        }
    }

    fn do_off(&mut self, x0: usize, x1: usize, y0: usize, y1: usize) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.update(x, y, -1)
            }
        }
    }

    fn do_toggle(&mut self, x0: usize, x1: usize, y0: usize, y1: usize) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.update(x, y, 2)
            }
        }
    }

    fn count_lights(&self) -> isize {
        let mut count = 0;
        for l in &self.vals {
            count += l;
        }

        count
    }
}

#[test]
fn test_parse() {
    let instructions = 
"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
    let parsed = parse(instructions);
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0], (Op::On, ((0, 999), (0, 999))));
    assert_eq!(parsed[1], (Op::Toggle, ((0, 999), (0, 0))));
    assert_eq!(parsed[2], (Op::Off, ((499, 500), (499, 500))));
}

#[test]
fn calc_numbers() {
    let g = BinaryGrid::new();

    // one number with offset 0
    let (x0, x1) = (64, 127);
    let nums = g.calc_numbers(x0, x1);
    assert_eq!(nums.len(), 1);
    assert_eq!(nums[0], u64::MAX);

    // one number, shifted
    let (x0, x1) = (13, 22);
    let nums = g.calc_numbers(x0, x1);
    assert_eq!(nums.len(), 1);
    assert_eq!(nums[0], (2_u64.pow(10) - 1) << 54 >> 13);

    // two numbers split with offset 13, length 13 + 63
    let (x0, x1) = (51, 126);
    let nums = g.calc_numbers(x0, x1);
    assert_eq!(nums.len(), 2);
    assert_eq!(nums[0], 2_u64.pow(13) - 1);
    assert_eq!(nums[1], (2_u64.pow(63) - 1) << 1);

    // four numbers starts at 22, ends at index
    let (x0, x1) = (790, 999);
    let nums = g.calc_numbers(x0, x1);
    assert_eq!(nums.len(), 4);
    assert_eq!(nums[0], 2_u64.pow(42) - 1);
    assert_eq!(nums[1], u64::MAX);
    assert_eq!(nums[2], u64::MAX);
    assert_eq!(nums[3], 2_u64.pow(40) - 1 << 24);

    let (x0, x1) = (790, 998);
    let nums = g.calc_numbers(x0, x1);
    assert_eq!(nums.len(), 4);
    assert_eq!(nums[0], 2_u64.pow(42) - 1);
    assert_eq!(nums[1], u64::MAX);
    assert_eq!(nums[2], u64::MAX);
    assert_eq!(nums[3], 2_u64.pow(39) - 1 << 25);
}   

#[test]
fn test_grid_handle_instr() {
    let mut g = BinaryGrid::new();

    let instr_on = (Op::On, ((0, 999), (0, 999)));
    g.handle_instr(instr_on);

    let instr_toggle = (Op::Toggle, ((0, 999), (0, 0)));
    g.handle_instr(instr_toggle);

    let instr_off = (Op::Off, ((499, 500), (499, 500)));
    g.handle_instr(instr_off);

    println!("{}", g);
}

#[test]
fn test_grid_count_lights() {
    let mut g = BinaryGrid::new();

    let instr_on = (Op::On, ((0, 999), (0, 999)));
    g.handle_instr(instr_on);

    assert_eq!(g.count_lights(), 1000000);
}