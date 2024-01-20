use clap::Parser;

#[derive(Parser, Debug)]
/// All cli arguments will be placed into this struct, to be able to be used in main
pub struct CliArgs {
    /// Which data / text file to execute
    pub input_file: String,

    /// The register's starting values (defaults to all 0x0000).
    #[arg(short = 'r')]
    pub reg_file: Option<String>,

    /// The IO's starting values (defaults to empty).
    #[arg(short = 'i')]
    pub io_file: Option<String>,

    /// The PC's starting value, where the .text section will start being placed (defaults to 0x0000).
    #[arg(short = 'p', default_value_t = 0x0000)]
    pub prog_counter: u16,

    /// The Memory's initial address, where the data section will start being placed (defaults to 0x4000).
    #[arg(short = 'm', default_value_t = 0x4000)]
    pub mem_init_addr: i16,
}
