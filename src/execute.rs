use::std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut}, mem::transmute,
};

use crate::{print_info, norm_n, Instructions};
use crate::parsing::ParseError;
use crate::spec::Instruction;

const INSTRUCTS_SLOW: [&str; 4] = ["LD", "LDB", "ST", "STB"];

impl Processador {
    /// Maximum allowed number of instructions to be run, to avoid generating infinite output wrt
    /// non-halting programs
    pub const MAX_INSTRUCTION_RUN_SIZE: usize = 10000;

    /// Create a new Processador given a starting state
    pub fn new(
        init_regs: Registers,
        init_mem: Memory,
        init_pc: ProgCounter,
        instructions: Instructions,
        init_io: HashMap<MemAddr, Value16Bit>,
    ) -> Self {
        Self {
            regs: init_regs,
            memory:init_mem,
            pc: init_pc,
            instr_memory: instructions,
            io: IOSystem(init_io),
            instrs_fetes: NumInstruccions::default(),
        }
    }
    #[rustfmt::skip]
    /// Execute any valid instruction directly, without going through the Program Counter
    pub fn execute_raw(&mut self, inst: &Instruction) {
        println!("[INFO]: Running \x1b[1;4;32m{:?}\x1b[0m", inst);

        if INSTRUCTS_SLOW.contains(&&*inst.get_verb()) {
            print_info("This instruction is SLOW (memory)"); 
            self.instrs_fetes.slow += 1;
        } else {
            print_info("This instruction is FAST (not-memory)"); 
            self.instrs_fetes.fast += 1;
        }

        match inst {
            Instruction::AND { a, b, d }      => self.regs[d].0 = self.regs[a].0 & self.regs[b].0,
            Instruction::OR { a, b, d }       => self.regs[d].0 = self.regs[a].0 | self.regs[b].0,
            Instruction::XOR { a, b, d }      => self.regs[d].0 = self.regs[a].0 ^ self.regs[b].0,
            Instruction::NOT { a, d }         => self.regs[d].0 = !self.regs[a].0,
            Instruction::ADD { a, b, d }      => self.regs[d] = self.regs[a] + self.regs[b],
            Instruction::ADDI { a, b, d }     => self.regs[d].0 = self.regs[a].0 + se_6(b.0),
            Instruction::SUB { a, b, d }      => self.regs[d] = self.regs[a] - self.regs[b],
            Instruction::SHA { a, b, d }      => self.regs[d] = self.regs[a].sha(self.regs[b]),
            Instruction::SHL { a, b, d }      => self.regs[d] = self.regs[a] << self.regs[b], // Implemented to do it using the last 5 bits
            Instruction::CMPEQ { a, b, d }    => self.regs[d].0 = (self.regs[a].0 == self.regs[b].0) as i16,
            Instruction::CMPLT  { a, b, d }   => self.regs[d].0 = (self.regs[a].0 < self.regs[b].0) as i16,
            Instruction::CMPLE  { a, b, d }   => self.regs[d].0 = (self.regs[a].0 <= self.regs[b].0) as i16,
            // TODO: Remove unnecssary use of transmute (use `as`)
            Instruction::CMPLTU { a, b, d }   => unsafe { self.regs[d].0 = (transmute::<i16, u16>(self.regs[a].0) < transmute(self.regs[b].0)) as i16 },
            Instruction::CMPLEU { a, b, d }   => unsafe { self.regs[d].0 = (transmute::<i16, u16>(self.regs[a].0) <= transmute(self.regs[b].0)) as i16 },
            Instruction::LD { a, d, offset }  => self.regs[d].0 = self.memory.get_word(&(se_6(offset.0) + self.regs[a].0).into()).unwrap_or_else(|| {
                print_info(&format!("Tried to access uninitialized memory (WORD) at addr: '{}' (hex 0x{0:X})", se_6(offset.0) + self.regs[a].0));
                panic!();
            }), 
            Instruction::LDB { a, d, offset } => self.regs[d].0 = se_8(self.memory.get_byte(&(se_6(offset.0) + self.regs[a].0).into()).unwrap_or_else(||{
                print_info(&format!("Tried to access uninitialized memory (BYTE) at addr: '{}' (hex 0x{0:X})", se_6(offset.0) + self.regs[a].0));
                panic!();
            })), 
            Instruction::ST  { a, b, offset } => self.memory.insert_word(&(self.regs[b].0 + se_6(offset.0)).into(), self.regs[a].0),
            Instruction::STB { a, b, offset } => self.memory.insert_byte(&(self.regs[b].0 + se_6(offset.0)).into(), (self.regs[a].0 & 0xF) as i8),
            Instruction::BZ  { a, offset }    => if self.regs[a].0 == 0 {self.pc.0 = (self.pc.0 as i16 + 2*se_8(offset.0)) as u16 }
            Instruction::BNZ { a, offset }    => if self.regs[a].0 != 0 {self.pc.0 = (self.pc.0 as i16 + 2*se_8(offset.0)) as u16 }
            Instruction::MOVI { d, n }        => self.regs[d].0 = se_8(n.0),
            Instruction::MOVHI { d, n }       => self.regs[d].0 |= (n.0 as i16) << 8,
            Instruction::IN { d, n }          => self.regs[d].0 = self.io.get(n).expect("Tried to access non existent IO address").0,
            Instruction::OUT { d, n }         => println!("[OUTPUT]: value '0x{0:0>4X}' ('{}') was printed on addr '{}'", self.regs[n].0, d),
            Instruction::JALR { a, d }        => { self.regs[d].0 = self.pc.0 as i16;   self.pc.0 = self.regs[a].0 as u16; }, // TODO: Test
            Instruction::NOP                  => {},
        }
        println!();
    }

    /// Execute the next instruction, which is the one that the Program  Counter is currently
    /// pointing to. If there is no instruction at that address, the program gracefully exits.
    pub fn execute_next(&mut self, print_status: bool) {
        print_info(&format!("Executing instruction at PC = {}", self.pc));
        let inst = self.instr_memory.get(&(self.pc.0 as i16).into());
        let inst = match inst {
            Some(i) => i.clone(),
            None => {
                println!("The number of instructions done is: {:?}", self.instrs_fetes);
                println!("There was no instruction to read when the PC = {} (dec '{}'), so the simulation has shut down 'gracefully' (for some definition of 'gracefully')",
                self.pc, self.pc.0);
                std::process::exit(0);
            },
        };
        self.pc.advance();
        self.execute_raw(&inst);
        if print_status { println!("{self}"); }
    }
    /// Update the IO's ports. Pretty much unusable as it must be hard-coded in
    pub fn update_io(&mut self, new_io: HashMap<MemAddr, Value16Bit>) { self.io = IOSystem(new_io); }
}

/// The sequence of eight registers that are contained in the [Processador]'s REGFILE.
pub struct Registers([Reg; 8]);

/// The held memory that is contained in the [Processador]'s MEMORY module, stored as bytes (not
/// words). 
#[derive(Debug, Clone, Default)] 
pub struct Memory(HashMap<MemAddr, MemValue>);

impl Memory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    /// Insert a byte at the given address
    pub fn insert_byte(&mut self, addr: &MemAddr, val: i8) {
        let _ = self.0.insert(addr.clone(), MemValue(val));
    }
    /// Insert a word at the given address in Little Endian: the even slot has the LSB and
    /// the even slot + 1 has MSB. Also note the alignment: if the address is odd, it will become
    /// even by truncation (`addr && !-1`)
    pub fn insert_word(&mut self, addr: &MemAddr, val: i16) {
        let high = ((val & 0xFF00u16 as i16) >> 7) as i8;
        let low = (val & 0x00FF) as i8;
        let addr = addr.align();

        self.0.insert(addr.clone(), MemValue(low));
        self.0.insert(MemAddr(addr.0 + 1), MemValue(high));
    }
    /// Get stored byte from the given memory address
    pub fn get_byte(&self, addr: &MemAddr) -> Option<i8> {
        self.0.get(addr).map(|m| m.0)
    }
    /// Get stored word from the given memory address. See the note about alignment at
    /// [insert_word](Memory::insert_word)
    pub fn get_word(&self, addr: &MemAddr) -> Option<i16> {
        let addr = addr.align();

        let low = self.0.get(&addr)?.0 as i16;
        let high = self.0.get(&MemAddr(addr.0 + 1))?.0 as i16;
        let out = (high << 7) | low;
        Some(out)
    }
}

impl MemAddr {
    /// Returns new address, but aligned instead. Does not require mutable access and instead
    /// returns the new value for better ergonomics when dealing with immutable addrs.
    fn align(&self) -> Self {
        //MemAddr(self.0 - (self.0 % 2)) // Equivalent but slower
        MemAddr(self.0 & !1)
    }
}

impl Default for Registers {
    fn default() -> Self {
        const EMPTY_REG: Reg = Reg(0);
        Self([EMPTY_REG; 8])
    }
}

impl Index<&RegLabel> for Registers {
    type Output = Reg;
    fn index(&self, index: &RegLabel) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<&RegLabel> for Registers {
    fn index_mut(&mut self, index: &RegLabel) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

/// The currently held values from the INPUT system. The output system is a rudimentary printing
/// out of the value and the intended address
pub struct IOSystem(HashMap<MemAddr, Value16Bit>);

impl IOSystem {
    fn get(&self, index: &MemAddr) -> Option<&Value16Bit> {
        self.0.get(index)
    }
    fn _get_mut(&mut self, index: &MemAddr) -> Option<&mut Value16Bit> {
        self.0.get_mut(index)
    }
}

/// The main Processor type. This contains the entire state of the simulator at any given time 
/// and implements all the functionality that is given.
///
/// Note that the instruction memory only holds its values at the even addresses and that the
/// [PC](ProgCounter)
/// increments by 2 on each one.
pub struct Processador {
    regs: Registers,
    memory: Memory,
    io: IOSystem,
    instr_memory: Instructions,
    pc: ProgCounter,
    instrs_fetes: NumInstruccions,
}

#[derive(Clone, Debug, Default)]
struct NumInstruccions {
    fast: usize,
    slow: usize,
}

#[rustfmt::skip] 
#[derive(Clone, Copy)]                    pub struct Reg(pub i16);
#[derive(Hash, PartialEq, Eq, Clone)] pub struct MemAddr(pub i16);
#[derive(Clone)]                     pub struct MemValue(pub i8);
#[derive(Clone)]                    pub struct MemOffset(pub i16);
/// The program counter, which hold the address of the next instruction to execute. It is
/// incremented by 2 on every instruction, and may be altered by special branching instructions.
#[derive(Clone)]                  pub struct ProgCounter(pub u16);
#[derive(Clone)]                   pub struct ImmediateN6(pub i8);
#[derive(Clone)]                   pub struct ImmediateN8(pub i8);
#[derive(Clone)]                   pub struct Value16Bit(pub i16);
#[derive(Debug, Clone)]              pub struct RegLabel(pub u8);

impl From<ProgCounter> for MemAddr {
    fn from(value: ProgCounter) -> Self {
        //Self(norm_n(&format!("{:X}", value.0)).unwrap() as i16)
        Self(value.0 as i16)
    }
}

impl From<i16> for MemAddr {
    fn from(value: i16) -> Self { Self(value) }
}

impl From<u16> for ProgCounter {
    fn from(value: u16) -> Self { Self(value) }
}

impl MemAddr {
    pub fn inc(&mut self)         { self.0 += 2; }
    pub fn inc_one(&mut self)     { self.0 += 1; }
    pub fn is_even(&self) -> bool { self.0 % 2 == 0 }
}

impl ProgCounter {
    /// Advance the address incrementing by 2. The increment is by 2 because instructions are a word
    /// long, and so are stored at the even addresses only
    pub fn advance(&mut self) {
        self.0 += 2;
    }
}

macro_rules! try_from_str_i8 {
    ($($name:ident),*$(,)?) => {
        $(
        impl TryFrom<&str> for $name {
            type Error = ParseError;
            fn try_from(val: &str) -> Result<Self, ParseError> {
                let n: i16 = norm_n(val)? as i16;
                Ok(Self(n as i8))
            }
        }
        )*
    }
}

macro_rules! try_from_str_i16 {
    ($($name:ident),*$(,)?) => {
        $(
        impl TryFrom<&str> for $name {
            type Error = ParseError;
            fn try_from(val: &str) -> Result<Self, ParseError> {
                let n: i16 = norm_n(val)? as i16;
                Ok(Self(n))
            }
        }
        )*
    }
}

macro_rules! other_impls_2 {
    ($($name:ident),*$(,)?) => {
        $(
            impl fmt::Display for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:0>2X}", self.0) } }
            impl fmt::Debug for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
        )*
    }
}

macro_rules! other_impls_4 {
    ($($name:ident),*$(,)?) => {
        $(
            impl fmt::Display for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:0>4X}", self.0) } }
            impl fmt::Debug for $name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }
        )*
    }
}

try_from_str_i8!(MemValue, ImmediateN6, ImmediateN8);
try_from_str_i16!(MemAddr, MemOffset);

other_impls_2!(MemValue, ImmediateN6, ImmediateN8);
other_impls_4!(MemAddr, MemOffset, ProgCounter);

impl fmt::Display for Processador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("[--------STATUS-------]: \n");
        out.push_str(&format!("- Regs: "));
        for (i, reg) in self.regs.0.iter().enumerate() {
            out.push_str(&format!("\x1b[1;4;31mR{i}: 0x{:0>4X}\x1b[0m,  ", reg.0));
        }
        out.push('\n');
        out.push_str(&format!("- Memory: \x1b[1;4;34m{:}\x1b[0m", self.memory));
        out.push_str("\n[-------END_STATUS-------]\n\n\n\n\n");

        write!(f, "{out}")
    }
}

// While the debug print of a HashMap works well enough, it is unordered. This makes actually using
// it kind of terrible, so this [fmt::Display] sorts it by address first and then prints it
impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let mut pairs: Vec<(i16, i8)> = self.0.iter().map(|(k, v)| (k.0, v.0)).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        for pair in pairs {
            out.push_str(&format!("0x{:0>4X}: 0x{:0>2X} | ", pair.0, pair.1));
        }


        write!(f, "{out}")
    }
}


fn se_6(n: i8) -> i16 {
    let n = n as i16;
    let val = if n < (1 << 5) { n } else { n | unsafe { transmute::<u16, i16>(0xFFC0) } };
    print_info(&format!("Sign extended 0x{:0>4X} into 0x{:0>4X}", n, val));

    val
}

fn se_8(n: i8) -> i16 {
    let n = n as i16;
    let val = if n < (1 << 7) { n } else { n | unsafe { transmute::<u16, i16>(0xFF00) } };
    //println!("[INFO]: \x1b[37mSign extended 0x{:0>4X} into 0x{:0>4X}\x1b[0m", n, val);
    print_info(&format!("Sign extended 0x{:0>4X} into 0x{:0>4X}", n, val));

    val
}

