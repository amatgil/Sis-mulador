use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut}, mem::transmute,
};

use crate::{norm_n, Instruction, ParseError};

impl Processador {
    pub fn new(
        init_regs: Registers,
        init_mem: HashMap<MemAddr, MemValue>,
        init_pc: ProgCounter,
        instructions: HashMap<MemAddr, Instruction>,
        init_io: HashMap<MemAddr, MemValue>,
    ) -> Self {
        Self {
            regs: init_regs,
            memory: init_mem,
            pc: init_pc,
            instr_memory: instructions,
            io: init_io,
        }
    }
    #[rustfmt::skip]
    pub fn execute_raw(&mut self, inst: &Instruction) {
        println!("[INFO]: Running {inst:?}");
        match inst {
            Instruction::AND { a, b, d }    => { self.regs[d].0 = self.regs[a].0 & self.regs[b].0; }
            Instruction::OR { a, b, d }     => { self.regs[d].0 = self.regs[a].0 | self.regs[b].0; }
            Instruction::XOR { a, b, d }    => { self.regs[d].0 = self.regs[a].0 ^ self.regs[b].0; }
            Instruction::NOT { a, d }       => self.regs[d].0 = !self.regs[a].0,
            Instruction::ADD { a, b, d }    => self.regs[d] = self.regs[a] + self.regs[b],
            Instruction::ADDI { a, b, d }   => self.regs[d].0 = self.regs[a].0 + b.0,
            Instruction::SUB { a, b, d }    => self.regs[d] = self.regs[a] - self.regs[b],
            Instruction::SHA { a, b, d }    => todo!(),
            Instruction::SHL { a, b, d }    => self.regs[d] = self.regs[a] << self.regs[b], // Implemented to do it using the last 5 bits
            Instruction::CMPEQ { a, b, d }  => { self.regs[d].0 = (self.regs[a].0 == self.regs[b].0) as usize }
            Instruction::CMPLTU { a, b, d } => { self.regs[d].0 = (self.regs[a].0 < self.regs[b].0) as usize }
            Instruction::CMPLEU { a, b, d } => { self.regs[d].0 = (self.regs[a].0 <= self.regs[b].0) as usize }
            Instruction::CMPLT  { a, b, d } => unsafe { self.regs[d].0 = (transmute::<usize, isize>(self.regs[a].0) < transmute(self.regs[b].0)) as usize }
            Instruction::CMPLE  { a, b, d } => unsafe { self.regs[d].0 = (transmute::<usize, isize>(self.regs[a].0) <= transmute(self.regs[b].0)) as usize }
            Instruction::LD { a, b, d }     => todo!(),
            Instruction::ST { a, b, d }     => todo!(),
            Instruction::LDB { a, b, d }    => todo!(),
            Instruction::STB { d, x, addr } => todo!(),
            Instruction::BZ { a, offset }   => todo!(),
            Instruction::BNZ { a, offset }  => todo!(),
            Instruction::MOVI { d, n }      => self.regs[d].0 = sign_extend(n).0,
            Instruction::MOVHI { d, n }     => todo!(),
            Instruction::IN { d, n }        => { self.regs[d].0 = self.io[n].0}
            Instruction::OUT { d, n }       => { println!("[OUTPUT]: value '0x{0:0>4X}' ('{}') was printed on addr '{}'", self.regs[d].0, n)}
            Instruction::NOP                => {}
        }
        println!();
    }
    pub fn execute_next(&mut self, print_status: bool) {
        println!("[INFO]: Executing instruction at PC = {}", self.pc);
        let inst = self.instr_memory[&MemAddr(self.pc.0)].clone();
        self.execute_raw(&inst);
        self.pc.advance();
        if print_status {
            println!("{self}");
        }
    }
    pub fn update_io(&mut self, new_io: HashMap<MemAddr, MemValue>) { self.io = new_io; }
}

pub struct Registers(pub [Reg; 8]);

impl Default for Registers {
    fn default() -> Self {
        const EMPTY_REG: Reg = Reg(0);
        Self([EMPTY_REG; 8])
    }
}

impl Index<&RegLabel> for Registers {
    type Output = Reg;
    fn index(&self, index: &RegLabel) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<&RegLabel> for Registers {
    fn index_mut(&mut self, index: &RegLabel) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

pub struct Processador {
    regs: Registers,
    memory: HashMap<MemAddr, MemValue>,
    io: HashMap<MemAddr, MemValue>,
    instr_memory: HashMap<MemAddr, Instruction>,
    pc: ProgCounter,
}

#[derive(Clone, Copy)]
pub struct Reg(pub usize);
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct MemAddr(pub usize);
#[derive(Clone)]
pub struct MemValue(pub usize);
#[derive(Clone)]
pub struct MemOffset(pub usize);
#[derive(Clone)]
pub struct ProgCounter(pub usize);
#[derive(Debug, Clone)]
pub struct RegLabel(pub u8);
#[derive(Clone)]
pub struct ImmediateN(pub usize);

impl From<ProgCounter> for MemAddr {
    fn from(value: ProgCounter) -> Self {
        Self(norm_n(&format!("{:X}", value.0)).unwrap())
    }
}

impl ProgCounter {
    pub fn advance(&mut self) {
        self.0 += 2;
    }
}

macro_rules! try_from_str {
    ($($name:ident),*$(,)?) => {
        $(
        impl TryFrom<&str> for $name {
            type Error = ParseError;
            fn try_from(val: &str) -> Result<Self, ParseError> {
                let n = norm_n(val)?;
                Ok(Self(n))
            }
        }
        )*
    }
}
macro_rules! other_impls {
    ($($name:ident),*$(,)?) => {
        $(
            impl fmt::Display for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:0>4X}", self.0) } }
            impl fmt::Debug for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
        )*
    }
}

try_from_str!(MemAddr, MemValue, MemOffset, ProgCounter, ImmediateN);
other_impls!(MemAddr, MemValue, MemOffset, ProgCounter, ImmediateN);

impl fmt::Display for Processador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("\n[--------STATUS-------]: \n");
        out.push_str(&format!("- Regs: "));
        for (i, reg) in self.regs.0.iter().enumerate() {
            out.push_str(&format!("R{i}: 0x{:0>4X} // ", reg.0));
        }
        out.push('\n');
        out.push_str(&format!("- Memory: {:?}", self.memory));
        out.push_str("\n[-------END_STATUS-------]\n");

        write!(f, "{out}")
    }
}

fn sign_extend(n: &MemValue) -> MemValue {
    let val = if n.0 < (1 << 7) { n.0 } else { n.0 | 0xFF00 };
    print!("[INFO]: Sign extended 0x{:0>4X} into 0x{:0>4X}", n.0, val);

    MemValue(val)
}
