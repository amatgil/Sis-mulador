#![allow(non_snake_case)]
use std::{collections::HashMap, env, convert::Infallible};

pub use sICmulador::{Instruction, ProgCounter};
use sICmulador::{MemAddr, Processador, Reg, Registers, Value16Bit, Memory, norm_n, read_instructions, FileError, ExecutionError, read_memory, print_info, cli::Args, read_io_once, read_registers};
use clap::Parser;


fn main() -> Result<Infallible, ExecutionError> {
    let args = Args::parse();

    let instructions = read_instructions(&args.instruction_file)?;
    let mem_name = args.memory_file;
    let memory: Memory = match mem_name {
        Some(f) => read_memory(&f)?,
        None => {
            print_info("No memory file provided, starting with empty memory");
            Memory::new()
        },
    };

    let io_system = match args.io_file {
        Some(f) => read_io_once(&f)?,
        None => {
            print_info("No IO file provided, starting without IO");
            HashMap::new()
        },
    };

    let init_pc: ProgCounter = ProgCounter(args.prog_counter.unwrap_or_default());
    let registers = match args.reg_file {
        Some(f) => read_registers(&f)?,
        None => {
            print_info("No initial registers file provided, starting with all 0");
            Registers::default()
        },
    };



    let mut cpu = Processador::new(
        registers,
        memory,
        init_pc,
        instructions,
        io_system,
    );
    println!("{cpu}");
    loop { cpu.execute_next(true); }
}

