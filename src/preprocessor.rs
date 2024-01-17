use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::ProgCounter;
use crate::{Instruction, Memory, Registers, Instructions, FileError, execute::MemAddr};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use anyhow::{Context, Result};

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


    let (memory, env) = dbg!(parse_directives(directives, mem_addr))?;
    let instructions = parse_instructions(text_area, env)?;


    Ok(Input {
        mem: memory,
        instructions,
    })

}

type Aliases = HashMap<String, String>;

fn parse_directives(directives: &str, mut mem_addr: MemAddr) -> anyhow::Result<(Memory,Aliases)> {
    let mut memory = Memory::new();

    let mut env: Aliases = HashMap::new();

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
                let n_items = parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?;
                let n_items: usize = n_items.parse()?;
                if let Some(item) = parts.next() {
                    memory.insert_byte(&mem_addr, item.parse()?); mem_addr.inc();
                    for _ in 0..n_items - 1 {
                        let val = parts.next().ok_or(ParsingError::MissingArgument { command: command.to_string() })?.parse()?;
                        memory.insert_byte(&mem_addr, val);
                        mem_addr.inc();
                    }
                }
            },
            ".even" => {},
            etiq => {
                let command: &str = parts.next().ok_or(ParsingError::MissingCommandAfterLabel { label: etiq.to_string() })?;
                match command {
                    ".byte" => {},
                    ".word" => {},
                    com => return Err(ParsingError::UnrecognizedCommandAfterLabel { label: etiq.into(), command: com.into() })?,
                }
            },
        }
    }

    Ok((
        memory,
        env,
    ))
}

fn parse_instructions(directives: &str, env: Aliases) -> anyhow::Result<Instructions> {
    todo!()
}
