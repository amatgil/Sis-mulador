use std::{collections::HashMap, path::{PathBuf, Path}, fs::File, io::Read, error::Error};

use toml::{Table, Value};

use crate::{MemAddr, Instruction, ExecutionError, Memory, norm_n, print_info, Value16Bit, Registers, RegLabel, Reg};

#[derive(Debug)]
pub enum FileError {
    FileNotFound,
    InstrucNotRecognized,
    ReadingError,
    UnparsableMemory,
    UnparsableIO,
    UnparsableRegister,
}
impl From<FileError> for ExecutionError {
    fn from(value: FileError) -> Self {
        Self::File(value)
    }
}
pub fn read_instructions(filename: &impl AsRef<Path>) -> Result<HashMap<MemAddr, Instruction>, FileError> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;

    let instructions = 
        contents.lines().enumerate()
        .map(|(i, line)| {
            //print_info("{i} -- {line}");
            match line.try_into() {
                Ok(ins) => Ok((MemAddr((i * 2) as i16), ins)),
                Err(_) => Err(FileError::InstrucNotRecognized),
            }
        }).collect::<Result<HashMap<MemAddr, Instruction>, FileError>>().or(Err(FileError::InstrucNotRecognized))?;

    Ok(instructions)
}


pub fn read_io_once(filename: &impl AsRef<Path>) -> Result<HashMap<MemAddr, Value16Bit>, FileError> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;
    let table: Table = contents.parse::<Table>().or(Err(FileError::UnparsableIO))?;

    let mut io = HashMap::new();
    for (m, v) in table.iter() {
        //print_info!("Pushing {m}, {v}");
        let v = v.as_str().ok_or(FileError::UnparsableIO)?;
        print_info(&format!("Pushing {}, {:X} to IO", &MemAddr(norm_n(m).unwrap() as i16), norm_n(v).unwrap() as i16));
        let _ = io.insert(
            MemAddr(norm_n(m).unwrap() as i16),
            Value16Bit(norm_n(v).unwrap() as i16));
    }
    Ok(io)
}

pub fn read_memory(filename: &impl AsRef<Path>) -> Result<Memory, FileError> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;
    let table: Table = contents.parse::<Table>().or(Err(FileError::UnparsableMemory))?;

    let mut memory = Memory::new();
    for (m, v) in table.iter() {
        //print_info!("Pushing {m}, {v}");
        let v = v.as_str().ok_or(FileError::UnparsableMemory)?;
        print_info(&format!("Pushing {}, {:X} to memory", &MemAddr(norm_n(m).unwrap() as i16), norm_n(v).unwrap() as i16));
        memory.insert_word(
            &MemAddr(norm_n(m).unwrap() as i16),
            norm_n(v).unwrap() as i16)

    }
    Ok(memory)
}

pub fn read_registers(filename: &impl AsRef<Path>) -> Result<Registers, FileError> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).or(Err(FileError::ReadingError))?;

    let mut registers = Registers::default();
    for (i, v) in contents.split(",").enumerate() {
        //print_info!("Pushing {m}, {v}");
        let Ok(v) = v.parse() else { return Err(FileError::UnparsableRegister) };
        registers[&RegLabel(i as u8)] = Reg(v);

    }
    Ok(registers)
}
