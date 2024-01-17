#![allow(non_snake_case)]
#![warn(missing_docs)]

#![doc = include_str!("../README.md")]

use std::{num::ParseIntError, fmt::Display, collections::HashMap};

type Instructions = HashMap<MemAddr, Instruction>;

mod calc;
mod execute;
mod parsing;
mod spec;
mod input;
mod cli;
mod preprocessor;

pub use input::*;
pub use execute::{Memory, IOSystem, Registers, ProgCounter, Processador};
pub use spec::Instruction;
pub use cli::CliArgs;
use spec::execute::MemAddr;

/// Main error enum for execution. Mostly seen at the start of execution.
#[derive(Debug)]
pub enum ExecutionError {
    /// Did not provide mandatory instruction file as an argument
    MissingInstructionsFile,

    /// Parsing related errors
    Parsing(ParseIntError),

    File(FileError),
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ExecutionError::MissingInstructionsFile=>"no instructions file was provided".into(),
            ExecutionError::Parsing(e)=>format!("encountered a parsing related error: {e}"),
            ExecutionError::File(f) => format!("encountered a file related error: {f:?}"),
        })
    }
}

impl std::error::Error for ExecutionError {}

pub(crate) fn norm_n(input: &str) -> Result<u16, ParseIntError> {
    let input = input.replace(",", "");
    if input.len() <= 2 || &input[..2] != "0x" { // Is dec here
        if input.chars().next().unwrap() == '-' {
            Ok(-((input[1..].parse::<u16>()?) as i16) as u16)
        } else { input.parse() }
    } else {
        u16::from_str_radix(&input[2..], 16)
    }
}

/// Print information in a darker, less noticeable color prefixed by `[INFO]:`
pub fn print_info(info: &str) { println!("[INFO]: \x1b[37m{}\x1b[0m",info); }

#[test]
fn test_norm() {
    use std::collections::HashMap;

    let pairs = HashMap::from([
        ("0x0000", 0),
        ("0x0001", 1),
        ("0x0010", 16),
        ("0x0015", 21),
        ("0x0020", 16 * 2),
        ("0", 0),
        ("1", 1),
        ("10", 10),
        ("15", 15),
        ("20", 20),
    ]);
    for (k, v) in pairs {
        dbg!(k, v);
        assert_eq!(norm_n(k), Ok(v));
    }
}


