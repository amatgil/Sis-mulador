use std::num::{ParseIntError, TryFromIntError};

use crate::*;

macro_rules! generate_parse_match {
    ($verb:ident, $parts:ident, $($name:ident),*$(,)?) => {
        Ok(match $verb {
            // Binary ones (AL, CMP)
            $(
                stringify!($name) => Instruction::$name {
                    d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                    a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                    b: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                },
            )*
            // Has no destination, is special
            "NOT" => Instruction::NOT  {
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?
            },
            "ADDI" => Instruction::ADDI {
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                b: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            },
            "BZ" => Instruction::BZ {
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                offset: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            },
            "BNZ" => Instruction::BNZ {
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                offset: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            },
            "MOVI" => Instruction::MOVI {
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                n: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            },
            "MOVHI" => Instruction::MOVHI {
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                n: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            },
            "ST" => Instruction::ST { // ST 20(R2), R0 <-> ST OFF(Rd), Ra
                offset: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
            },
            "LD" => Instruction::LD { // LD R0, 20(R2) <-> ST Rd, OFF(Ra)
                d: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                offset: $parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
                a: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
            },
            x => return Err(ParseError::UnrecognizedInstruction(x.into()))
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingVerb,
    MissingImmediate,
    MissingDestination,
    MissingReg,
    EmptyVerb,
    ParseInt(ParseIntError),
    TryFromInt(TryFromIntError),
    RegLabel(RegLabelError),
    UnrecognizedInstruction(String),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
impl From<TryFromIntError> for ParseError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromInt(value)
    }
}
impl From<RegLabelError> for ParseError {
    fn from(value: RegLabelError) -> Self {
        Self::RegLabel(value)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, ParseError> {
        let value = value.replace("(", " ");
        let mut parts = value.split(" ");
        let verb = parts.next().ok_or(ParseError::MissingVerb)?;
        println!("[INFO]: Verb parsed is: {verb}");

        // Special cases
        if verb == "NOP" {
            return Ok(Instruction::NOP);
        } else if verb == "OUT" {
            return Ok(Instruction::OUT {
                d: parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                n: parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            });
        } else if verb == "IN" {
            return Ok(Instruction::IN {
                d: parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                n: parts.next().ok_or(ParseError::MissingImmediate)?.try_into()?,
            });
        }

        generate_parse_match!(
            verb, parts, AND, OR, XOR, ADD, SUB, SHA, SHL, CMPLT, CMPLE, CMPEQ, CMPLTU, CMPLEU
        )
    }
}

#[derive(Debug)]
pub enum RegLabelError {
    MissingNumber,
    UnrecognizedNumber,
}
impl TryFrom<&str> for RegLabel {
    type Error = RegLabelError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut char = input.bytes().nth(1).ok_or(RegLabelError::MissingNumber)?;
        if char == b'(' {char = input.bytes().nth(2).ok_or(RegLabelError::MissingNumber)?;}
        let n = if char.is_ascii_alphanumeric() {
            char - '0' as u8
        } else {
            return Err(RegLabelError::UnrecognizedNumber);
        };
        eprintln!("Interpreting '{input}' as R'{n}'");
        Ok(RegLabel(n))
    }
}
