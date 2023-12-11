pub mod parsing;
pub mod spec;

use std::{collections::HashMap, fmt::{Display, self}};

pub use spec::*;
pub use parsing::*;

#[derive(Clone, Copy)]
pub struct Reg(pub usize);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct MemAddr(pub usize);

#[derive(Clone, Copy)]
pub struct MemValue(pub usize);

#[derive(Clone, Copy)]
pub struct PC(pub usize);

pub struct Processador {
    regs: [Reg; 8],
    memory: HashMap<MemAddr, MemValue>,
    pc: PC,
}


impl From<&str> for MemAddr { fn from(val: &str) -> Self { Self(norm_n(val)) } }
impl From<&str> for MemValue { fn from(val: &str) -> Self { Self(norm_n(val)) } }
impl From<&str> for PC { fn from(val: &str) -> Self { Self(norm_n(val)) } }

impl Display for MemAddr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:x}", self.0) } }
impl Display for MemValue { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:x}", self.0) } }
impl Display for PC { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:x}", self.0) } }

impl fmt::Debug for MemAddr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
impl fmt::Debug for MemValue { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
impl fmt::Debug for PC { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }

impl Display for Processador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (i, reg) in self.regs.iter().enumerate() { out.push_str(&format!("R{i}: {}", reg.0)); }
        out.push('\n');
        out.push_str(&format!("{:?}", self.memory));

        write!(f, "{out}")
    }
}

fn norm_n(input: &str) -> usize {
    if input.len() <= 2 || &input[..2] != "0x" { input.parse().unwrap() } // Is dec here
    else {  usize::from_str_radix(&input[2..], 16).expect("Numero hex invàlid") }
}

impl Processador {
    pub fn new(init_regs: [Reg; 8], init_mem: HashMap<MemAddr, MemValue>, init_pc: PC) -> Self {
        Self { regs: init_regs, memory: init_mem, pc: init_pc, }
    }
    pub fn execute(&mut self, inst: &Instruction) {
        match inst {
            _ => todo!(),
        }
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
        assert_eq!(norm_n(k), v);
    }
}
