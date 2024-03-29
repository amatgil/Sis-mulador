use std::{collections::HashMap, path::Path, fs::File, io::Read};

use toml::Table;

use crate::{execute::{Reg, RegLabel, Registers, Value16Bit, MemAddr}, print_info, norm_n, PreparationError, spec::Instruction, Instructions, Memory};

/// Describes all variants of filesystem errors, for using in [ExecutionError]
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum FileError {
    #[error("could not find file")]
    FileNotFound,
    #[error("found non-recognized instruction")]
    InstrucNotRecognized,
    #[error("error while reading in the file bytes")]
    ReadingError,
    #[error("memory file is not properly written")]
    UnparsableMemory,
    #[error("IO file is not properly written")]
    UnparsableIO,
    #[error("registers' file is not properly written")]
    UnparsableRegister,
}

impl From<FileError> for PreparationError {
    fn from(value: FileError) -> Self {
        Self::File(value)
    }
}

/// Read the instructions file when ran with --simple
pub fn read_simple_instructions_file(filename: &impl AsRef<Path>) -> anyhow::Result<Instructions> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;

    read_instructions(&contents)
}

/// Get a previous preprocessed file and turn it into a usable ordered instruction set to be
/// simulated
pub fn read_instructions(input: &str) -> anyhow::Result<Instructions> {
    let instructions = 
        input.lines().enumerate()
        .map(|(i, line)| {
            let line = line.trim();
            match line.try_into() {
                Ok(ins) => Ok((MemAddr((i * 2) as i16), ins)),
                Err(e) => {
                    print_info(&format!("Did not recognize instruction: '{}' with error: {e:?}", line));
                    Err(FileError::InstrucNotRecognized)
                },
            }
            // TODO: Rewrite... whatever this is
        }).collect::<Result<HashMap<MemAddr, Instruction>, FileError>>().or(Err(FileError::InstrucNotRecognized))?; 

    Ok(instructions)
}


/// Read IO settings from file, in the TOML format. They must be separated by newlines. 
///
/// For example:
/// ```txt
/// 1 = "0x0001"
/// 0 = "0x0005"
/// ```
/// To represent `KEY-STATUS` as 1 and `KEY-DATA` as 5
pub fn read_io_once(filename: &impl AsRef<Path>) -> anyhow::Result<HashMap<MemAddr, Value16Bit>> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;
    let table: Table = contents.parse::<Table>().or(Err(FileError::UnparsableIO))?;

    let mut io = HashMap::new();
    for (m, v) in table.iter() {
        let v = v.as_str().ok_or(FileError::UnparsableIO)?;
        let _ = io.insert(MemAddr(norm_n(m).unwrap() as i16), Value16Bit(norm_n(v).unwrap() as i16));
    }
    Ok(io)
}

/// Read memory list from file, in the TOML format. They must be separated by newlines. 
///
/// For example:
/// ```txt
/// 0
/// 0
/// 0
/// 0
/// 0
/// 0
/// 10
/// 5
///```
pub fn read_registers(filename: &impl AsRef<Path>) -> anyhow::Result<Registers> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;

    let mut registers = Registers::default();
    for (i, v) in contents.lines().enumerate() {
        let Ok(v) = v.parse() else { return Err(FileError::UnparsableRegister)? };
        print_info(&format!("R{i} = 0x{:X} (dec {0})", v));
        registers[&RegLabel(i as u8)] = Reg(v);
    }
    Ok(registers)
}

/// Read memory list from file, in the TOML format. They must be separated by newlines. 
///
/// For example:
/// ```txt
/// 0x0022 = "0x0000"
/// 0x0024 = "0x0002"
/// 0x0026 = "0xFFFB"
/// 0x0028 = "0x0108"
/// 0x002A = "0xFF9D"
/// 0x002C = "0x0017"
/// 0x002E = "0x003A"
/// 0x0030 = "0xFF9C"
/// 0x0032 = "0x0020"
/// 0x0034 = "0x0000"
/// 0x0036 = "0xFFF9"
/// ```
pub fn read_memory(filename: &impl AsRef<Path>) -> anyhow::Result<Memory> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;
    let table: Table = contents.parse::<Table>().or(Err(FileError::UnparsableMemory))?;

    let mut memory = Memory::new();
    for (m, v) in table.iter() {
        let v = v.as_str().ok_or(FileError::UnparsableMemory)?;
        memory.insert_word( &MemAddr(norm_n(m).unwrap() as i16), norm_n(v).unwrap() as i16)
    }
    Ok(memory)
}
