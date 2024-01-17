use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::{ProgCounter, Memory, Instructions, FileError, execute::MemAddr};
use nom::{IResult, bytes::complete::{tag, take_until}};
use anyhow::Context;

const DEFAULT_SPACE_FILLER_VALUE: i8 = 0;

#[derive(Debug, thiserror::Error)]
enum ParsingError {
    #[error("missing argument for command: {command}")]
    MissingArgument {
        command: String
    },
    #[error("command not recognized: {command}")]
    UnrecognizedCommand {
        command: String
    },
    #[error("command not recognized after label '{label}': {command}")]
    UnrecognizedCommandAfterLabel {
        label: String,
        command: String
    },
    #[error("command missing after label '{label}'")]
    MissingCommandAfterLabel {
        label: String,
    },
}

pub struct Input {
    pub mem: Memory,
    pub instructions: Instructions
}

/// The requirements for the file are quite particular. They are:
/// - The first line must be '.data', followed by a sequence of newline separated directives
/// (listed below)
/// - 
pub fn parse_file(filename: &str, mem_addr: MemAddr, instr_addr: ProgCounter) -> anyhow::Result<Input> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut input = String::new();
    input_file.read_to_string(&mut input).context("could not read from file")?;
    
    let data_tag: IResult<&str, &str> = tag(".data")(&*input);
    let (input, _) = data_tag.map_err(|e| e.to_owned()).context("input does not start with '.data'")?;
    let directives: IResult<&str, &str> = take_until(".text")(input);
    dbg!(&input, &directives);
    let (input, directives) = directives.map_err(|e| e.to_owned()).context("could not parse the directives")?;

    let text_tag: IResult<&str, &str> = tag(".text")(input);
    let (input, _) = text_tag.map_err(|e| e.to_owned()).context("input does not start with '.data'")?;
    let text_area: IResult<&str, &str> = take_until(".end")(input);
    let (_input, text_area) = text_area.map_err(|e| e.to_owned()).context("could not parse the data section")?;


    let (memory, env, ptrs) = parse_directives(directives, mem_addr)?;
    println!("{memory}"); dbg!(&env, &ptrs);
    let instructions = parse_instructions(text_area, env, ptrs, instr_addr)?;


    Ok(Input {
        mem: memory,
        instructions,
    })

}

type Aliases = HashMap<String, String>;
type Pointers = HashMap<String, MemAddr>;

fn parse_directives(directives: &str, mut mem_addr: MemAddr) -> anyhow::Result<(Memory,Aliases, Pointers)> {
    let mut memory = Memory::new();

    let mut env:   Aliases = HashMap::new();
    let mut ptrs: Pointers = HashMap::new();

    for line in directives.lines().filter(|l| !l.is_empty()) {
        let line = line.trim();
        eprintln!("Parsing directive: {line}");
        let mut parts = line.split(" ");
        let command = parts.next().unwrap(); // SAFETY: We've filtered out empty lines
        match command {
            ".set" => { env.insert(
                parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?.into(),
                parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?.into());
            },
            ".space" => {
                let n_items = parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?.parse()?;
                if let Some(item) = parts.next() {
                    memory.insert_byte(&mem_addr, item.parse()?); mem_addr.inc();
                    for _ in 0..n_items - 1 {
                        let val = parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?.parse()?;
                        memory.insert_byte(&mem_addr, val);
                        mem_addr.inc();
                    }
                } else {
                    for _ in 0..n_items {
                        memory.insert_byte(&mem_addr, DEFAULT_SPACE_FILLER_VALUE);
                        mem_addr.inc();
                    }
                }
            },
            ".even" => { if !mem_addr.is_even() { mem_addr.inc_one();} },
            etiq => {
                let etiq = &etiq[0..etiq.len() - 1]; // Remove colon
                let command: &str = parts.next().ok_or(ParsingError::MissingCommandAfterLabel { label: etiq.to_string() })?;
                match command {
                    ".byte" => {
                        ptrs.insert(etiq.into(), mem_addr.clone());
                        let bytes: Vec<_> = line.split(" ").map(|b| b.parse::<i8>()).collect();
                        for byte in bytes {
                            let byte = byte?;
                            memory.insert_byte(&mem_addr, byte);
                            mem_addr.inc_one();
                        }
                    },
                    ".word" => {
                        ptrs.insert(etiq.into(), mem_addr.clone());
                        let words: Vec<_> = parts.map(|b| b.parse::<i16>()).collect();
                        for word in words {
                            let word = word?;
                            memory.insert_word(&mem_addr, word);
                            mem_addr.inc();
                        }
                    },
                    com => return Err(ParsingError::UnrecognizedCommandAfterLabel { label: etiq.into(), command: com.into() })?,
                }
            },
        }
    }

    Ok((
        memory,
        env,
        ptrs
    ))
}

fn parse_instructions(text: &str, mut env: Aliases, mut ptrs: Pointers, mut pc: ProgCounter) -> anyhow::Result<Instructions> {
    let mut labels_addrs = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(colon_idx) = line.find(':') {
            let etiq = &line[0..colon_idx];
            labels_addrs.insert(etiq.to_string(), pc.clone());
            eprintln!("Line '{line}' has label '{etiq}' at addr {}", pc);
            pc.advance();
        }
    }

    todo!()
}
