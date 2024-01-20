#![allow(non_snake_case)]
use std::{collections::HashMap, convert::Infallible};

use clap::Parser;
use sICmulador::{*, preprocessor::{parse_file, Input}};
pub use sICmulador::CliArgs;


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

    let Input { mem: memory, instructions } = parse_file(&args.input_file, args.mem_init_addr.into(), args.prog_counter.into())?;


    let mut cpu = Processador::new(
        registers,
        memory,
        init_pc,
        instructions,
        io_system,
    );
    print_info("\n\nStarting with state:");
    println!("{cpu}");

    print_info("Starting execution...");
    loop { cpu.execute_next(true); }
}

