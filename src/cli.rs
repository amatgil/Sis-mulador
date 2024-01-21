use clap::Parser;

#[derive(Parser, Debug)]
/// All cli arguments will be placed into this struct, to be able to be used in main
pub struct CliArgs {
    /// Which instructions file to execute. If --simple is set, it must be labelless and have no data section. 
    /// If --simple is not set, there must be a .data/.text/.end section
    pub input_file: String,

    /// The register's starting values (defaults to all 0x0000).
    #[arg(short = 'r', long)]
    pub reg_file: Option<String>,

    /// The IO's starting values (defaults to empty).
    #[arg(short = 'i', long)]
    pub io_file: Option<String>,

    /// The PC's starting value, where the .text section will start being placed (defaults to 0x0000).
    #[arg(short = 'p', default_value_t = 0x0000, long = "pc")]
    pub prog_counter: u16,

    /// The Memory's initial address, where the data section will start being placed (defaults to 0x4000).
    #[arg(short = 'm', default_value_t = 0x4000, long = "mem")]
    pub mem_init_addr: i16,

    /// The memory's starting state (defaults to empty). Note that the instruction memory and the 
    /// data memory are considered separate and may overlap. Only used if `--simple` is used
    ///
    /// Note that both memories are separate internally to avoid chaos (and implementing a `binary
    /// -> instruction` parser).
    #[arg(long = "mfile")]
    pub memory_file: Option<String>,

    /// Whether or not to use the simplistic input model, where the instructions file has no labels
    /// and no data/text section and the memory is provided separately.
    ///
    /// When set to false: only a single input file may be used, the .data/.text file which is
    /// allowed to contain labels and use dot directives. No separate memory file may be provided,
    /// causing the program to abort if this occurs.
    /// 
    /// When set to true: two input files may be used: the instructions file (which may NOT contain
    /// labels of any king, or directives) and the initial memory state in toml format as shown in
    /// the docs.
    #[arg(short, long, default_value_t = false)]
    pub simple: bool,
}
