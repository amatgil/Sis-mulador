#![allow(non_snake_case)]

#[cfg(not(feature = "executable"))]

fn main() {
    compile_error!("Please use the 'executable' feature (cargo build -F executable) to run the program");
}

#[cfg(feature = "executable")]
use std::{collections::HashMap, convert::Infallible};
use clap::Parser;
use sICmulador::{*, preprocessor::{Input, parse_complete_file}};
pub use sICmulador::CliArgs;


#[cfg(feature = "executable")]
fn main() -> anyhow::Result<Infallible> {
    let args = CliArgs::parse();

    let io_system = match args.io_file {
        Some(f) => read_io_once(&f)?,
        None => {
            print_info("No IO file provided, starting without IO");
            HashMap::new()
        },
    };
    let init_pc: ProgCounter = ProgCounter(args.prog_counter);
    let registers = match args.reg_file {
        Some(f) => read_registers(&f)?,
        None => {
            print_info("No initial registers file provided, starting with all 0");
            Registers::default()
        },
    };

    let mut cpu = if args.simple {
        let instructions = read_simple_instructions_file(&args.input_file)?;
        let memory = if let Some(mem_file) = args.memory_file { read_memory(&mem_file)? }
            else { Memory::default() };

        Processador::new( registers, memory, init_pc, instructions, io_system)

    } else {
        if args.memory_file.is_some() { 
            eprintln!("Initial memory file was provided, but no --simple flag: aborting");
            std::process::exit(1);
        }
        let Input { mem: memory, instructions } = parse_complete_file(&args.input_file, args.mem_init_addr.into(), args.prog_counter.into())?;

        Processador::new( registers, memory, init_pc, instructions, io_system)

    };

    print_info("\n\nStarting with state:");
    println!("{cpu}");

    let mut instructions_executed = 0;

    print_info("Starting execution...");
    loop {
        cpu.execute_next(true);
        instructions_executed += 1;
        if instructions_executed >= Processador::MAX_INSTRUCTION_RUN_SIZE {
            println!("{} insturctions have been executed, so it seems like the program is non-halting. For the \
                     sake of your terminal and your hard-drive, execution has been stoped", Processador::MAX_INSTRUCTION_RUN_SIZE);
            std::process::exit(0);
        }
    }
}

