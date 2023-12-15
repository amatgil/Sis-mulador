use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Which instructions to execute
    pub instruction_file: String,

    /// The memory's starting to use (defaults to empty). Note that the instruction memory and the 
    /// data memory are considered separate and may overlap.
    #[arg(short = 'm')]
    pub memory_file: Option<String>,

    /// The register's starting values (defaults to all 0x0000).
    #[arg(short = 'r')]
    pub reg_file: Option<String>,

    /// The IO's starting values (defaults to empty).
    #[arg(short = 'i')]
    pub io_file: Option<String>,

    /// The PC's starting value (defaults to 0x0000).
    #[arg(short = 'p')]
    pub prog_counter: Option<u16>,
}
