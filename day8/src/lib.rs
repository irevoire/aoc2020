#[derive(Debug, Clone)]
pub struct Vm {
    pub mem: Vec<Instruction>,
    pub pc: isize,
    pub acc: isize,
}

impl Vm {
    pub fn new(mem: Vec<Instruction>) -> Self {
        Self { mem, pc: 0, acc: 0 }
    }

    /// execute one cycle and return the position of the program counter or None if the program is
    /// finished
    pub fn cycle(&mut self) -> Option<usize> {
        use Instruction::*;
        match self.mem.get(self.pc as usize)? {
            Nop(_) => self.pc += 1,
            Acc(i) => {
                self.acc += i;
                self.pc += 1;
            }
            Jmp(i) => self.pc += i,
        }
        Some(self.pc as usize)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl std::str::FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(2, char::is_whitespace);
        let ins = s.next().unwrap();
        let i = s.next().unwrap().parse()?;

        Ok(match ins {
            "nop" => Instruction::Nop(i),
            "acc" => Instruction::Acc(i),
            "jmp" => Instruction::Jmp(i),
            _ => unreachable!(),
        })
    }
}
