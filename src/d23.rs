fn parse(input: &'static str) -> Vec<Instr> {
    input.lines()
         .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>().into())
         .collect::<Vec<Instr>>()
}

#[derive(Clone, Copy)]
enum Instr {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

impl Instr {
    fn exec(self, machine: &mut Machine) {
        match self {
            Instr::Hlf(reg) => machine.exec_hlf(reg),
            Instr::Tpl(reg) => machine.exec_tpl(reg),
            Instr::Inc(reg) => machine.exec_inc(reg),
            Instr::Jmp(offset) => machine.exec_jmp(offset),
            Instr::Jie(reg, offset) => machine.exec_jie(reg, offset),
            Instr::Jio(reg, offset) => machine.exec_jio(reg, offset),
        }
    }
}

impl From<Vec<&str>> for Instr {
    fn from(parts: Vec<&str>) -> Self {
        if parts.len() == 2 {
            match parts[0] {
                "hlf" => Self::Hlf(parts[1].into()),
                "tpl" => Self::Tpl(parts[1].into()),
                "inc" => Self::Inc(parts[1].into()),
                "jmp" => Self::Jmp(parts[1].parse().unwrap()),
                _ => panic!(),
            }
        } else {
            let reg =  Reg::from(parts[1]);
            let offset = parts[2].parse().unwrap();
            match parts[0] {
                "jie" => Self::Jie(reg, offset),
                "jio" => Self::Jio(reg, offset),
                _ => panic!(),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Reg {
    A,
    B,
}

impl From<&str> for Reg {
    fn from(reg_name: &str) -> Self {
        if reg_name.starts_with('a') { Reg::A } else { Reg::B }
    }
}

struct Machine {
    a: usize,
    b: usize,
    instructions: Vec<Instr>,
    ptr: usize,
}

impl Machine {
    fn new(instructions: Vec<Instr>) -> Self {
        Self { a: 0, b: 0, instructions, ptr: 0 }
    }

    fn exec_instruction(&mut self) -> Option<()> {
        let instr = self.instructions[self.ptr];
        instr.exec(self);
        if self.ptr >= self.instructions.len() {
            None
        } else {
            Some(())
        }
    }

    fn exec_hlf(&mut self, reg: Reg) {
        match reg {
            Reg::A => self.a /= 2,
            Reg::B => self.b /= 2,
        }
        self.set_ptr(1);
    }

    fn exec_tpl(&mut self, reg: Reg) {
        match reg {
            Reg::A => self.a *= 3,
            Reg::B => self.b *= 3,
        }
        self.set_ptr(1);
    }

    fn exec_inc(&mut self, reg: Reg) {
        match reg {
            Reg::A => self.a += 1,
            Reg::B => self.b += 1,
        }
        self.set_ptr(1);
    }   

    fn exec_jmp(&mut self, offset: isize) {
        self.set_ptr(offset);
    }

    fn exec_jie(&mut self, reg: Reg, offset: isize) {
        let reg_val = self.get_reg_val(reg);

        if reg_val % 2 == 0 {
            self.set_ptr(offset);
        } else {
            self.set_ptr(1);
        }
    }

    fn exec_jio(&mut self, reg: Reg, offset: isize) {
        let reg_val = self.get_reg_val(reg);

        if reg_val == 1 {
            self.set_ptr(offset);
        } else {
            self.set_ptr(1);
        }
    }

    fn get_reg_val(&self, reg: Reg) -> usize {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
        }
    }

    fn set_ptr(&mut self, offset: isize) {
        self.ptr = (self.ptr as isize + offset) as usize;
    }
}

pub fn get_solution_1() -> usize {
    let instructions = parse(include_str!("../data//d23.txt"));
    let mut machine = Machine::new(instructions);

    while let Some(_) = machine.exec_instruction() {}

    machine.b
}

pub fn get_solution_2() -> usize {
    let instructions = parse(include_str!("../data//d23.txt"));
    let mut machine = Machine::new(instructions);

    machine.a = 1;

    while let Some(_) = machine.exec_instruction() {}

    machine.b
}

#[test]
fn test_machine() {
    let instructions = parse(include_str!("../data//d23_test.txt"));
    let mut machine = Machine::new(instructions);

    while let Some(_) = machine.exec_instruction() {}

    println!("{}", machine.a);
}