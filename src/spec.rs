use crate::{MemAddr, MemOffset};

#[derive(Debug, Clone)]
pub enum Instruction {
    /// OP == 00
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

    /// OP == 01
    CMPLT{
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLE{
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPEQ{
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLTU{
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    CMPLEU{
        a: RegLabel,
        b: RegLabel,
        d: RegLabel,
    },
    ADDI {
        a: RegLabel,
        b: ImmediateN,
        d: RegLabel,
    },
    LD {
        a: RegLabel,
        b: ImmediateN,
        d: RegLabel,
    },
    ST {
        a: RegLabel,
        b: RegLabel,
        d: ImmediateN,
    },
    LDB {
        a: RegLabel,
        b: ImmediateN,
        d: RegLabel,
    },
    STB {
        d: RegLabel,
        x: RegLabel,
        addr: ImmediateN,
    },
    BZ {
        a: RegLabel,
        offset: ImmediateN,
    },
    BNZ {
        a: RegLabel,
        offset: ImmediateN,
    },
    MOVI { 
        d: RegLabel,
        n: MemAddr,
    },
    MOVHI { 
        d: RegLabel,
        n: MemAddr,
    },
    IN {
        d: RegLabel,
        n: ImmediateN,
    },
    OUT {
        d: RegLabel,
        n: ImmediateN,
    },
    NOP, 

}

#[derive(Debug, Clone, Copy)]
pub struct RegLabel(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct ImmediateN(pub usize);

