use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Which instructions to execute
    pub instruction_file: String,
    /// Which starting to use (optional),
    pub memory_file: Option<String>,
    /// The register's starting values (defaults to all 0x0000),
    pub reg_file: Option<String>,
    /// The IO's starting values (defaults to empty),
    pub io_file: Option<String>,
    /// The PC's starting value (defaults to 0x0000),
    pub prog_counter: Option<u16>,
}
