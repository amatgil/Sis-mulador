#[derive(Debug)]
pub enum Instruction {
    /// OP == 00
    AND {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    OR {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    XOR {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    NOT {
        destination: Option<RegLabel>,
        x: RegLabel,
    },
    ADD {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    SUB {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    SHA {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    SHL {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },

    /// OP == 01
    CMPLT{
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    CMPLE{
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    CMPEQ{
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    CMPLTU{
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    CMPLEU{
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    ADDI {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    LD {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    ST {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    LDB {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    STB {
        destination: Option<RegLabel>,
        x: RegLabel,
        y: Data,
    },
    BZ {
        x: RegLabel,
        y: Data,
    },
    BNZ {
        x: RegLabel,
        y: Data,
    },
    MOVI { 
        destination: Option<RegLabel>,
        y: Data,
    },
    MOVHI { 
        destination: Option<RegLabel>,
        y: Data,
    },
    IN {
        destination: RegLabel,
    },
    OUT {
        x: Data,
    },
    NOP, 

}

#[derive(Debug)]
pub enum Data {
    Reg(RegLabel),
    Immediate(i16)
}
#[derive(Debug, Clone, Copy)]
pub struct RegLabel(pub u8);
