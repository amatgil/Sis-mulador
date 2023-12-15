use std::{collections::HashMap, path::{PathBuf, Path}, fs::File, io::Read, error::Error};

use toml::{Table, Value};

use crate::{MemAddr, Instruction, ExecutionError, Memory, norm_n, print_info};

#[derive(Debug)]
pub enum FileError {
    FileNotFound,
    InstrucNotRecognized,
    ReadingError,
    UnparsableMemory,
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