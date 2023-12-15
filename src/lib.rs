#![allow(non_snake_case)]
pub mod calc;
pub mod execute;
pub mod parsing;
pub mod spec;
pub mod input;

pub use calc::*;
pub use execute::*;
pub use parsing::*;
pub use spec::*;
pub use input::*;
pub use std::num::ParseIntError;

#[derive(Debug)]
pub enum ExecutionError {
    MissingInstructionsFile,
    MissingMemoryFile,
    File(FileError),
}

pub fn norm_n(input: &str) -> Result<u16, ParseIntError> {
    if input.len() <= 2 || &input[..2] != "0x" { // Is dec here
        if input.chars().next().unwrap() == '-' {
            Ok(
                -((input[1..].parse::<u16>()?) as i16) as u16
            )
        } else {
            input.parse()
        }
    } else {
        u16::from_str_radix(&input[2..], 16)
    }
}

pub fn print_info(info: &str) {
    println!("[INFO]: \x1b[37m{}\x1b[0m",info);
}

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
