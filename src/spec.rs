pub use crate::*;

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
        b: ImmediateN,
        d: RegLabel,
    },
    LD {
        d: RegLabel,
        a: RegLabel,
        offset: ImmediateN,
    },
    LDB {
        d: RegLabel,
        a: RegLabel,
        offset: ImmediateN,
    },
    ST {
        a: RegLabel,
        offset: ImmediateN,
        d: RegLabel,
    },
    STB {
        a: RegLabel,
        offset: ImmediateN,
        d: RegLabel,
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
        n: MemValue,
    },
    MOVHI {
        d: RegLabel,
        n: MemValue,
    },
    IN {
        d: RegLabel,
        n: MemAddr,
    },
    OUT {
        d: RegLabel,
        n: MemAddr,
    },
    NOP,
}
