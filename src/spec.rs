pub use crate::*;

use self::execute::{RegLabel, ImmediateN6, ImmediateN8, MemAddr};

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Instruction {
    AND {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    OR {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    XOR {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    NOT {
        a: RegLabel,
        d: RegLabel,
    },
    ADD {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    SUB {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    SHA {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    SHL {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLT {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLE {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPEQ {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLTU {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLEU {
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    ADDI {
        a: RegLabel,
        b: ImmediateN6,
        d: RegLabel,
    },
    LD {
        d: RegLabel,
        a: RegLabel,
        offset: ImmediateN6,
    },
    LDB {
        d: RegLabel,
        a: RegLabel,
        offset: ImmediateN6,
    },
    ST {
        offset: ImmediateN6,
        a: RegLabel,
        b: RegLabel,
    },
    STB {
        offset: ImmediateN6,
        a: RegLabel,
        b: RegLabel,
    },
    BZ {
        a: RegLabel,
        offset: ImmediateN8,
    },
    BNZ {
        a: RegLabel,
        offset: ImmediateN8,
    },
    MOVI {
        d: RegLabel,
        n: ImmediateN8,
    },
    MOVHI {
        d: RegLabel,
        n: ImmediateN8,
    },
    IN {
        d: RegLabel,
        n: MemAddr,
    },
    OUT {
        d: MemAddr,
        n: RegLabel,
    },
    NOP,
}

impl Instruction {
    /// Extract the verb that the instruction uses. 
    ///
    /// For example, in `ADD R1, R2, R3`, the verb would be `ADD`
    pub fn get_verb(&self) -> String {
        match self {
            Instruction::AND    { .. } => "AND",
            Instruction::OR     { .. } => "OR",
            Instruction::XOR    { .. } => "XOR",
            Instruction::NOT    { .. } => "NOT",
            Instruction::ADD    { .. } => "ADD",
            Instruction::SUB    { .. } => "SUB",
            Instruction::SHA    { .. } => "SHA",
            Instruction::SHL    { .. } => "SHL",
            Instruction::CMPLT  { .. } => "CMPLT",
            Instruction::CMPLE  { .. } => "CMPLE",
            Instruction::CMPEQ  { .. } => "CMPEQ",
            Instruction::CMPLTU { .. } => "CMPLTU",
            Instruction::CMPLEU { .. } => "CMPLEU",
            Instruction::ADDI   { .. } => "ADDI",
            Instruction::LD     { .. } => "LD",
            Instruction::LDB    { .. } => "LDB",
            Instruction::ST     { .. } => "ST",
            Instruction::STB    { .. } => "STB",
            Instruction::BZ     { .. } => "BZ",
            Instruction::BNZ    { .. } => "BNZ",
            Instruction::MOVI   { .. } => "MOVI",
            Instruction::MOVHI  { .. } => "MOVHI",
            Instruction::IN     { .. } => "IN",
            Instruction::OUT    { .. } => "OUT",
            Instruction::NOP           => "NOP",
        }.into()
    }
}
