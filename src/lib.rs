#![allow(non_snake_case)]
#![warn(missing_docs)]
//! 
//! A fully fledged SISA instruction interpreter and simulator.
//!
//! The code is incredibly self documenting (on purpose), using the newtype pattern wherever
//! helpfuly. Usage of the binary is below.
//! 
//! (the ergonomics aren't the best, but it works Well Enough)
//! 
//! ## Example usage
//! To solve the alien that I had for 13c (which gave instuctions and initial memory, with an initial
//! PC of 0 and no initial register state), use:
//! ```rs
//! cargo run -- examples/alien/alien.sisa -m examples/alien/alien.smem -i examples/alien/alien.sio
//! ```
//! 
//! For the `mul16` algorithm:
//! ```rs
//! cargo run -- examples/mul16/mul16.sisa -r examples/mul16/mul16.sregs
//! ```
//! 
//! Or for the `mulfast` algorithm:
//! ```rs
//! cargo run -- examples/mulfast/mulfast.sisa -r examples/mulfast/mulfast.sregs
//! ```
//! 
//! Both multiplication algorithms will multiply 5 times 10, giving `0x32` on `R5`.
//! 
//! 
//! ## IO
//! Use `[cpu].update_io(new_io)` to change the IO status in between `execute`s. This cannot be done
//! from the cli, at the moment, because I have no idea how (and it doesn't seem that useful, to be 
//! honest).
//! 
//! ## Registers
//! File must contain eight lines (or less, for a computer with less registers), each with a decimal number. E.g.
//! ```txt
//! 0
//! 0
//! 0
//! 0
//! 0
//! 0
//! 5
//! 10
//! ```
//! 
//! Register `6` will hold `0x5` and Register `7` will hold `0xA`.
//! 
//! You cannot use more than eight registers, it will panic (this is intended behavior).
//! 
//! ## NOTE
//! It assumes the input is wellformed. Do not feed it instuctions like
//! 
//! `MOVI R5, 0x555`
//! 
//! because it's invalid. If you feed it invalid input, you're gonna get UB (good luck lmao).
//! 
//! Also, the memory and data memories are separate because I didn't stop to think before I started 
//! writing down code. Just, like, assume they're the same. I'm protecting you from yourself (this actually
//! has already helped me).
//! 
//! ## Roadmap
//! Use `as` more (note the sign extension remarks when upcasting): [Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)

use std::num::ParseIntError;

mod calc;
mod execute;
mod parsing;
mod spec;
mod input;
mod cli;

pub use input::*;
pub use execute::{Memory, IOSystem, Registers, ProgCounter, Processador};
pub use spec::Instruction;
pub use cli::CliArgs;

/// Main error enum for execution. Mostly seen at the start of execution.
#[derive(Debug)]
pub enum ExecutionError {
    /// Did not provide mandatory instruction file as an argument
    MissingInstructionsFile,

    /// Filesystem related errors
    File(FileError),
}

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
