#![allow(non_snake_case)]
use std::{collections::HashMap, env, convert::Infallible};

pub use SISA_sim::{Instruction, ProgCounter};
use SISA_sim::{MemAddr, Processador, Reg, Registers, Value16Bit, Memory, norm_n, read_instructions, FileError, ExecutionError, read_memory, print_info};


fn main() -> Result<Infallible, ExecutionError> {
    let args: Vec<String> = env::args().collect();
    let program_name = args.get(1).ok_or(ExecutionError::MissingInstructionsFile)?;
    let instructions = read_instructions(program_name)?;
    let mem_name = args.get(2);

    let memory: Memory = match mem_name {
        Some(f) => read_memory(f)?,
        None => {
            print_info("No memory file provided, starting with empty memory");
            Memory::new()
        },
    };

    dbg!(&memory);

    let io_system: HashMap<MemAddr, Value16Bit> = HashMap::from([
        (MemAddr(0), Value16Bit(0x0001)), // Key Status == 0
        (MemAddr(1), Value16Bit(0x0005)), // Key Data == 1
    ]);
    let init_pc: ProgCounter = ProgCounter(0);


    let mut cpu = Processador::new(
        Registers([
            Reg(0),
            Reg(0),
            Reg(0),
            Reg(0),
            Reg(0),
            Reg(0),
            Reg(0x0081),
            Reg(0x0005),
        ]),
        memory,
        init_pc,
        instructions,
        io_system,
    );
    println!("{cpu}");
    loop { cpu.execute_next(true); }
}

