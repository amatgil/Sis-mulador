use std::{collections::HashMap, mem};
use std::fs::File;
use std::io::Read;

use crate::{read_instructions, print_info};
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
    _UnrecognizedCommand { // Unused because if the command isn't recognized it's assumed to be a
                           // label lmao, check the below errors for more details
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

/// The struct returned by parse_file, it contains the memory and instructions specified in the
/// .data and .text section of the file respectively.
pub struct Input {
    /// The memory, as specified by the .data section
    pub mem: Memory,
    /// The instructions, as specified by the .text section
    pub instructions: Instructions
}

/// The requirements for the file are quite particular. They are:
/// - The first line must be '.data', followed by a sequence of newline separated directives
///     - Type A: `.set LENGTH 8`, `.even`, `.space 10 FF`
///     - Type B: `v: .byte 1 4 6 3 2 4 6 7 4 2 1`, `x: .word 6 4 2 2 5 6 7 4 31`
/// - After the .data section comes the '.text' section, which contains the instructions
/// - The last line must be `.end`. If there's content after it, it will not be lead
///
/// Note that comments work as usual with `;`.
pub fn parse_complete_file(filename: &str, mem_addr: MemAddr, instr_addr: ProgCounter) -> anyhow::Result<Input> {
    let mut input_file = File::open(filename).or(Err(FileError::FileNotFound))?;
    let mut input = String::new();
    input_file.read_to_string(&mut input).context("could not read from file")?;

    input = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line.trim().split(';');
            let mut l = line.next().unwrap().to_string(); // SAFETY: We're removed empty lines, each line must contain
                                                          // _something_
            l.push('\n');
            l
        }).collect();
    println!("Input is: {input}");
    
    let data_tag: IResult<&str, &str> = tag(".data")(&input);
    let (input, _) = data_tag.map_err(|e| e.to_owned()).context("input does not start with '.data': did you forget to use `--simple`?")?;
    let directives: IResult<&str, &str> = take_until(".text")(input);
    let (input, directives) = directives.map_err(|e| e.to_owned()).context("could not parse the directives")?;

    let text_tag: IResult<&str, &str> = tag(".text")(input);
    let (input, _) = text_tag.map_err(|e| e.to_owned()).context("input does not start with '.data'")?;
    let text_area: IResult<&str, &str> = take_until(".end")(input);
    let (_input, text_area) = text_area.map_err(|e| e.to_owned()).context("could not parse the data section, .end may be missing")?;


    let (memory, env, ptrs) = parse_directives(directives, mem_addr)?;
    println!("Preprocessed memory is:");
    println!("-----------------------\n");
    println!("{memory}");
    println!("-----------------------\n");
    let instructions = parse_instructions(text_area, &env, ptrs, &instr_addr)?;


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

    for line in directives.lines().filter(|line| !line.is_empty()) {
        let line = line.trim();
        print_info(&format!("Parsing directive: {line}"));
        let mut parts = line.split(' ');
        let command = parts.next().unwrap(); // SAFETY: We've filtered out empty lines earlier
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
                        let bytes: Vec<_> = line.split(' ').map(|b| b.parse::<i8>()).collect();
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

// 'Aliases' are String -> String maps, like `SIZE := 7`; 'Pointers' are labels
// This is INCREDIBLE inefficient, there's a lot of reallocation and copying and whatever, but it
// doesn't really matter
fn parse_instructions(text: &str, env: &Aliases, mut ptrs: Pointers, pc: &ProgCounter) -> anyhow::Result<Instructions> {
    // THE PLAN:
    // Do it in passes, changing things like `lo(v)` for their value n things. When it's all
    // neat and tidy, run it by the function in `parsing.rs` :)

    // First pass: gather and remove all the etiquetes
    // Second pass: change out lo(x), high(x) and labels for their values
    //  - First check if it's a label. If it's not, check if it's an alias (labels take preference)
    // Third parse: full parsing

    let original_text = text.to_string();
    let mut first_pc = pc.clone();

    // First pass
    for line in original_text.clone().lines().filter(|l| !l.is_empty()) {
        let line = line.trim();
        if let Some(colon_idx) = line.find(':') {
            let etiq = &line[0..colon_idx];
            ptrs.insert(etiq.to_string(), first_pc.clone().into());
            print_info(&format!("PARSING: Line '{line}' has label '{etiq}' at addr {}", first_pc));
        } else {
            first_pc.advance();
        }
    }

    mem::drop(first_pc);
    let mut labelless_text: Vec<String> = original_text.clone()
        .lines()
        .filter(|l| !l.is_empty())
        .filter(|l| l.find(':').is_none())
        .map(|l| l.to_string())
        .collect();

    for (i, line) in labelless_text.iter_mut().enumerate() {
        *line = line
            .split(' ')
            .map(|word| { // Get lo() and hi()
                if word.len() < 3 { word.into() }
                else if &word[0..3] == "lo(" { get_part_of_label(word, &ptrs, PartOfAddr::Lo) }
                else if &word[0..3] == "hi(" { get_part_of_label(word, &ptrs, PartOfAddr::Hi) }
                else { word.into() }
            })
            .map(|word| { // The aliases
                if let Some(aliased) = env.get(&word) { aliased.clone() } 
                else { word }
            })
            .map(|word| { // The labels thing (requires MATH!! WATCH OUT!!!11!!!!111!!)
                if let Some(target_addr) = ptrs.get(&word) {
                    let curr_addr = pc.0 + (i as u16)*2;
                    let delta = (target_addr.0 - curr_addr as i16) / 2 - 1; // Minus 2 because BZ
                                                                            // adds to PC + 2
                    format!("0x{:X}", delta)
                } else { word }
            })
            .collect::<Vec<_>>()
            .join(" ");
    }
    let processed_text: String = labelless_text.join("\n");

    println!("Preprocessed text is:");
    println!("-----------------------\n");
    println!("{processed_text}");
    println!("-----------------------\n");
    read_instructions(&processed_text)
}

enum PartOfAddr {
    Lo,
    Hi
}

fn get_part_of_label(word: &str, ptrs: &Pointers, part: PartOfAddr) -> String {
    let label = word[3..word.len() - 1].to_string();
    let value = ptrs.get(&label).expect(&format!("Tried to use label '{label}', but it did not exist"));
    let value: i16 = match part {
        PartOfAddr::Lo => value.0 & 0x00FF,
        PartOfAddr::Hi => (value.0 & 0xFF00u16 as i16) >> 8,
    };
    format!("0x{:X}", value)
}

