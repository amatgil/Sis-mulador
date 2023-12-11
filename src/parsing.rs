use std::num::ParseIntError;

use crate::*;

macro_rules! generate_parse_match {
    ($verb:ident, $is_immediate:ident, $destination:ident, $parts:ident, $($name:ident),*$(,)?) => {
        Ok(match ($verb, $is_immediate) {
            // Binary ones
            $(
                (stringify!($name), true) => Instruction::$name {
                    destination: $destination,
                    x: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                    y: Data::Immediate(i16::from_str_radix(&$parts.next().ok_or(ParseError::MissingImmediate)?[2..], 16)? as i16)
                },
                (stringify!($name), false) => Instruction::$name {
                    destination: $destination,
                    x: $parts.next().ok_or(ParseError::MissingReg)?.try_into()?,
                    y: Data::Reg(RegLabel::try_from($parts.next().ok_or(ParseError::MissingReg)?)?)
                },
            )*
            //Edge cases
            ("NOT", _) => Instruction::NOT  { $destination, x: $parts.next().ok_or(ParseError::MissingReg)?.try_into()? }, // There is no immediate NOT

            // Extra-taula
            ("MOVE", true) => Instruction::MOVE { destination: $destination.expect("Tried to move to nowhere"), x: Data::Immediate(u16::from_str_radix(&$parts.next().ok_or(ParseError::MissingImmediate)?[2..], 16)? as i16)},
            ("MOVE", false) => Instruction::MOVE { destination: $destination.expect("Tried to move to nowhere"), x: Data::Reg(RegLabel::try_from($parts.next().ok_or(ParseError::MissingReg)?)?)},
            (v, i) => panic!("Erroring out, can't parse: '{v}, {i}'"),
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
    RegLabelError(RegLabelError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self { Self::ParseInt(value) }
}

impl From<RegLabelError> for ParseError {
    fn from(value: RegLabelError) -> Self {
        Self::RegLabelError(value)
    }
}


impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, ParseError> {
        let mut parts = value.split(" ");
        let mut verb = parts.next().ok_or(ParseError::MissingVerb)?;
        let is_immediate = verb.bytes().last().ok_or(ParseError::EmptyVerb)? == b'I'; // For NOT, this must always be false

        // Special cases
        if verb == "NOP" { return Ok(Instruction::NOP) } 
        else if verb == "OUT" {
            if is_immediate {
                return Ok(Instruction::OUT { x: Data::Immediate(parts.next().ok_or(ParseError::MissingImmediate)?.parse()?)});
            } else {
                return Ok(Instruction::OUT { x: Data::Reg(parts.next().ok_or(ParseError::MissingReg)?.try_into()?) });
            }
        } else if verb == "IN" {
            return Ok(Instruction::IN { destination: parts.next().ok_or(ParseError::MissingDestination)?.try_into()?});
        }

        let destination: Option<RegLabel> = match parts.next().ok_or(ParseError::MissingDestination)? {
            "-" => None,
            s => Some(s.try_into()?),
        };
        dbg!(destination);

        eprint!("INFO: Verb was: {verb}");
        if is_immediate {verb = &verb[..verb.len() - 1]};
        eprintln!(".. but is now {verb}");

        dbg!(&parts.clone().collect::<Vec<_>>());
        generate_parse_match!(verb, is_immediate, destination, parts,
           AND, OR, XOR, ADD, SUB, SHA, SHL, CMPLT, CMPLE, CMPEQ, CMPLTU, CMPLEU)
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
        let char = input.bytes().nth(1).ok_or(RegLabelError::MissingNumber)?;
        let n = if char.is_ascii_alphanumeric() { char - '0' as u8 } else {return Err(RegLabelError::UnrecognizedNumber);};
        eprintln!("Interpreting '{input}' as R'{n}'");
        Ok(RegLabel(n))
    }
}



