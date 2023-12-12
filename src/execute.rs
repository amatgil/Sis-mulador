use std::{collections::HashMap, fmt};

use crate::{Instruction, norm_n, ParseError};


pub struct Processador {
    regs: [Reg; 8],
    memory: HashMap<MemAddr, MemValue>,
    inst_memory: HashMap<MemAddr, Instruction>,
    pc: ProgCounter,
}

#[derive(Clone)] pub struct Reg(pub usize);
#[derive(Hash, PartialEq, Eq, Clone)] pub struct MemAddr(pub usize);
#[derive(Clone)] pub struct MemValue(pub usize);
#[derive(Clone)] pub struct MemOffset(pub usize);
#[derive(Clone)] pub struct ProgCounter(pub usize);
#[derive(Debug, Clone)] pub struct RegLabel(pub u8);
#[derive(Clone)] pub struct ImmediateN(pub usize);

impl From<ProgCounter> for MemAddr { fn from(value: ProgCounter) -> Self { Self(norm_n(&format!("{:X}", value.0)).unwrap()) } }

impl ProgCounter {
    pub fn advance(&mut self) { self.0 += 2; }
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
        let mut out = String::from("\n[STATUS]: \n");
        out.push_str(&format!("- Regs: "));
        for (i, reg) in self.regs.iter().enumerate() { out.push_str(&format!("R{i}: 0x{:0>4X} // ", reg.0)); }
        out.push('\n');
        out.push_str(&format!("- Memory: {:?}", self.memory));
        out.push_str("\n[END_STATUS]\n");

        write!(f, "{out}")
    }
}
impl Processador {
    pub fn new(init_regs: [Reg; 8], init_mem: HashMap<MemAddr, MemValue>, init_pc: ProgCounter, instructions: HashMap<MemAddr, Instruction>) -> Self {
        Self { regs: init_regs, memory: init_mem, pc: init_pc, inst_memory: instructions }
    }
    pub fn execute_raw(&mut self, inst: &Instruction) {
        println!("[INFO]: Running {inst:?}");
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
    pub fn execute_next(&mut self, print_status: bool) {
        println!("[INFO]: Executing instruction at PC = {}", self.pc);
        let inst = self.inst_memory[&MemAddr(self.pc.0)].clone();
        self.execute_raw(&inst);
        self.pc.advance();
        if print_status {println!("{self}");}
    }
}

fn sign_extend(n: &MemValue) -> MemValue {
    let val = if n.0 < (1 << 7) { n.0 } else { n.0 | 0xFF00 };
    print!("[INFO]: Sign extended 0x{:0>4X} into 0x{:0>4X}", n.0, val);

    MemValue(val)
}


