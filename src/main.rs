use std::{collections::HashMap, default, mem::transmute};

pub use SISA_sim::{Instruction, ProgCounter};
use SISA_sim::{MemAddr, MemValue, Processador, Reg, Registers, Value16Bit, Memory, norm_n};

const SAMPLE_INPUT: &str = 
"IN R0, 0
BZ R0, -2
IN R0, 1
MOVI R2, 0x24
MOVHI R2, 0x00
MOVI R3, 0x22
MOVHI R3, 0x00
MOVI R1, 7
MOVI R4, 1
LD R5, 0(R2)
CMPLT R6, R5, R0
BZ R6, 2
ST 20(R2), R0
STB 0(R3), R4
ADDI R2, R2, 2
ADDI R1, R1, -1
BNZ R1, -8";

fn main() {
    let pre_memory: HashMap<&str, &str> = HashMap::from([
        ("0x0022", "0x0000"),
        ("0x0024", "0x0002"),
        ("0x0026", "0xFFFB"),
        ("0x0028", "0x0108"),
        ("0x002A", "0xFF9D"),
        ("0x002C", "0x0017"),
        ("0x002E", "0x003A"),
        ("0x0030", "0xFF9C"),
        ("0x0032", "0x0020"),
        ("0x0034", "0x0000"),
        ("0x0036", "0xFFF9"),
    ]);
    
    let mut memory = Memory::new();
    pre_memory.iter().for_each(|(m, v)| {
        println!("Pushing {m}, {v}");
        println!("Meaning, Pushing {}, {:X}", &MemAddr(norm_n(m).unwrap() as i16), norm_n(v).unwrap() as i16);
        memory.insert_word(
            &MemAddr(norm_n(m).unwrap() as i16),
            norm_n(v).unwrap() as i16)
    });
    dbg!(&memory);

    let io_system: HashMap<MemAddr, Value16Bit> = HashMap::from([
        (MemAddr(0), Value16Bit(0x0001)), // Key Status == 0
        (MemAddr(1), Value16Bit(0x0005)), // Key Data == 1
    ]);
    let init_pc: ProgCounter = ProgCounter(0);

    let instructions: HashMap<MemAddr, Instruction> = 
        SAMPLE_INPUT.lines().enumerate()
        .map(|(i, line)| {
            println!("{i} -- {line}");
            (MemAddr((i * 2) as i16), line.try_into().unwrap())
        }).collect();
    dbg!(&instructions);

    let mut cpu = Processador::new(
        Registers([
            Reg(0),
            Reg(2),
            Reg(unsafe{ transmute(-2_i16) }),
            Reg(3),
            Reg(unsafe{ transmute(-3_i16) }),
            Reg(4),
            Reg(unsafe{ transmute(-5_i16) }),
            Reg(0),
        ]),
        memory,
        init_pc,
        instructions,
        io_system,
    );
    println!("{cpu}");
    loop { cpu.execute_next(true); }
}

