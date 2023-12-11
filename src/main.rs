use std::collections::HashMap;

use SISACompiler::{Reg, PC, MemAddr, MemValue, Processador};

fn main() {
    let mut memory: HashMap<MemAddr, MemValue> = HashMap::from([
        ("0x0022".into(), "0x0000".into()),
        ("0x0024".into(), "0x0002".into()),
        ("0x0026".into(), "0xFFFB".into()),
        ("0x0028".into(), "0x0108".into()),
        ("0x002A".into(), "0xFF9D".into()),
        ("0x002C".into(), "0x0017".into()),
        ("0x002E".into(), "0x003A".into()),
        ("0x0030".into(), "0xFF9C".into()),
        ("0x0032".into(), "0x0020".into()),
        ("0x0034".into(), "0x0000".into()),
        ("0x0036".into(), "0xFFF9".into()),
    ]);

    let init_pc: PC = "0x0000".into();
    let init_regs: [Reg; 8] = [Reg(0); 8];

    let mut cpu = Processador::new(init_regs, memory, init_pc);
    println!("{cpu}");
    cpu.execute(&"MOVI R2, 0x24".try_into().unwrap());
    println!("{cpu}");
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


