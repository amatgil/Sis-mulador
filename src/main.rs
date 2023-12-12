
use std::{collections::HashMap, default};

use SISA_sim::{MemAddr, MemValue, Reg, Processador, Registers};
pub use SISA_sim::{ProgCounter, Instruction};


//use SISACompiler::{Reg, prog_counter, MemAddr, MemValue, Processador, Instruction};

fn main() {
    let memory: HashMap<MemAddr, MemValue> = HashMap::from([
       /*
        ("0x0022".try_into().unwrap(), "0x0000".try_into().unwrap()),
        ("0x0024".try_into().unwrap(), "0x0002".try_into().unwrap()),
        ("0x0026".try_into().unwrap(), "0xFFFB".try_into().unwrap()),
        ("0x0028".try_into().unwrap(), "0x0108".try_into().unwrap()),
        ("0x002A".try_into().unwrap(), "0xFF9D".try_into().unwrap()),
        ("0x002C".try_into().unwrap(), "0x0017".try_into().unwrap()),
        ("0x002E".try_into().unwrap(), "0x003A".try_into().unwrap()),
        ("0x0030".try_into().unwrap(), "0xFF9C".try_into().unwrap()),
        ("0x0032".try_into().unwrap(), "0x0020".try_into().unwrap()),
        ("0x0034".try_into().unwrap(), "0x0000".try_into().unwrap()),
        ("0x0036".try_into().unwrap(), "0xFFF9".try_into().unwrap()),
        */
    ]);

    let init_pc: ProgCounter = ProgCounter(0);

    let instructions: HashMap<MemAddr, Instruction> = HashMap::from([
        //(MemAddr(0), "MOVI R2, 0x24".try_into().unwrap()),
        (MemAddr(0), "AND R0, R1, R3".try_into().unwrap()),
        (MemAddr(2), "OR R0, R1, R3".try_into().unwrap()),
        (MemAddr(4), "XOR R0, R1, R3".try_into().unwrap()),
        (MemAddr(6), "MOVI R0, 0x94".try_into().unwrap()),
        (MemAddr(8), "CMPEQ R0, R1, R2".try_into().unwrap()),
        (MemAddr(10), "CMPEQ R0, R1, R1".try_into().unwrap()),
    ]);

    let mut cpu = Processador::new(
        Registers([Reg(0), Reg(20), Reg(53), Reg(21343), Reg(523542), Reg(414), Reg(0), Reg(0)]),
        memory, init_pc, instructions);
    println!("{cpu}");
    loop { cpu.execute_next(true); }
}

const INPUT: &str =
"IN R0, 0001
BZ R0, -2
IN R0, 0005
MOVI R2, 0x24
MOVHI R2, 0x00
MOVI R3, 0x22
MOVHI R3,0x00
MOVI R1, 7
MOVI R4, 1
LD R5, 0(R2)
CMPLT R6, R5, R0
BZ R6, 2
ST 20(R2), R0
STB 0(R3), R4
ADDI R2,R2,2
ADDI R1,R1,-1
BNZ R1, -8";


