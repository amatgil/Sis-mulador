pub mod parsing;
pub mod spec;

use std::{collections::HashMap, fmt::{Display, self}, num::{ParseIntError, TryFromIntError}};

pub use spec::*;
pub use parsing::*;

#[derive(Clone, Copy)]
pub struct Reg(pub usize);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct MemAddr(pub usize);

#[derive(Clone, Copy)]
pub struct MemValue(pub usize);

#[derive(Clone, Copy)]
pub struct MemOffset(pub usize);

#[derive(Clone, Copy)]
pub struct PC(pub usize);

pub struct Processador {
    regs: [Reg; 8],
    memory: HashMap<MemAddr, MemValue>,
    inst_memory: HashMap<MemAddr, Instruction>,
    pc: PC,
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
            impl Display for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:0>4X}", self.0) } }
            impl fmt::Debug for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
        )*
    }
}
try_from_str!(MemAddr, MemValue, MemOffset, PC, ImmediateN);
other_impls!(MemAddr, MemValue, MemOffset, PC);

impl Display for Processador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (i, reg) in self.regs.iter().enumerate() { out.push_str(&format!("R{i}: 0x{:0>4X} // ", reg.0)); }
        out.push('\n');
        out.push_str(&format!("{:?}", self.memory));

        write!(f, "{out}")
    }
}

fn norm_n(input: &str) -> Result<usize, ParseIntError> {
    if input.len() <= 2 || &input[..2] != "0x" { input.parse() } // Is dec here
    else {  usize::from_str_radix(&input[2..], 16) }
}

impl Processador {
    pub fn new(init_regs: [Reg; 8], init_mem: HashMap<MemAddr, MemValue>, init_pc: PC, instructions: HashMap<MemAddr, Instruction>) -> Self {
        Self { regs: init_regs, memory: init_mem, pc: init_pc, inst_memory: instructions }
    }
    pub fn execute_raw(&mut self, inst: &Instruction) {
        println!("Running {inst:?}");
        match inst {
            Instruction::AND { a, b, d } => todo!(),
            Instruction::OR { a, b, d } => todo!(),
            Instruction::XOR { a, b, d } => todo!(),
            Instruction::NOT { a, d } => todo!(),
            Instruction::ADD { a, b, d } => todo!(),
            Instruction::SUB { a, b, d } => todo!(),
            Instruction::SHA { a, b, d } => todo!(),
            Instruction::SHL { a, b, d } => todo!(),
            Instruction::CMPLT { a, b, d } => todo!(),
            Instruction::CMPLE { a, b, d } => todo!(),
            Instruction::CMPEQ { a, b, d } => todo!(),
            Instruction::CMPLTU { a, b, d } => todo!(),
            Instruction::CMPLEU { a, b, d } => todo!(),
            Instruction::ADDI { a, b, d } => todo!(),
            Instruction::LD { a, b, d } => todo!(),
            Instruction::ST { a, b, d } => todo!(),
            Instruction::LDB { a, b, d } => todo!(),
            Instruction::STB { d, x, addr } => todo!(),
            Instruction::BZ { a, offset } => todo!(),
            Instruction::BNZ { a, offset } => todo!(),
            Instruction::MOVI { d, n } => {
                let n = sign_extend(n);
                self.regs[d.0 as usize].0 = n.0;
            }
            Instruction::MOVHI { d, n } => todo!(),
            Instruction::IN { d, n } => todo!(),
            Instruction::OUT { d, n } => todo!(),
            Instruction::NOP => todo!(),
        }
        println!();
    }
    pub fn execute_next(&mut self) {
        println!("Executing instruction at PC = {}", self.pc);
        let inst = self.inst_memory[&self.pc.into()].clone();
        self.execute_raw(&inst);
        self.pc.advance();
    }
}

fn sign_extend(n: &MemValue) -> MemValue {
    print!("Sign extended 0x{:0>4X}", n.0);
    let val = if n.0 < (1 << 7) { n.0 } else { n.0 + 0xFF00 };
    println!(" into 0x{:0>4X}", n.0);

    MemValue(val)
}

impl From<PC> for MemAddr {
    fn from(value: PC) -> Self {
        Self(norm_n(&format!("{:X}", value.0)).unwrap())
    }
}
impl PC {
    fn advance(&mut self) {
        self.0 += 2;
    }
}

#[test]
fn test_norm() {
    let pairs = HashMap::from([
        ("0x0000", 0), ("0x0001", 1), ("0x0010", 16),
        ("0x0015", 21), ("0x0020", 16*2), ("0", 0),
        ("1", 1), ("10", 10), ("15", 15), ("20", 20),
    ]);
    for (k, v) in pairs {
        dbg!(k, v);
        assert_eq!(norm_n(k), Ok(v));
    }
}
