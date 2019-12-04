use crate::instructions;
use crate::instructions::{ InstrThumb16 };
use crate::memory::{ RegisterBank, Memory };

/// ARMv7-M virtual processor
/// 
/// Registers:
/// [ R0  ]: General purpose Thumb16 addressable
/// [ R1  ]: General purpose Thumb16 addressable
/// [ R2  ]: General purpose Thumb16 addressable
/// [ R3  ]: General purpose Thumb16 addressable
/// [ R4  ]: General purpose Thumb16 addressable
/// [ R5  ]: General purpose Thumb16 addressable
/// [ R6  ]: General purpose Thumb16 addressable
/// [ R7  ]: General purpose Thumb16 addressable
/// [ R8  ]: General purpose
/// [ R9  ]: General purpose
/// [ R10 ]: General purpose
/// [ R11 ]: General purpose
/// [ R12 ]: General purpose
/// [ R13 ]: Stack Pointer
/// [ R14 ]: Link Register
/// [ R15 ]: Program Counter
pub struct Processor {
    dct: [InstrThumb16; instructions::NUM_TH16_INSTRUCTIONS],
    reg: RegisterBank,
    mem: Memory,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            dct: [InstrThumb16::Undefined; instructions::NUM_TH16_INSTRUCTIONS],
            reg: RegisterBank::new(),
            mem: Memory::new()
        }
    }

    pub fn init(&mut self) {
        self.dct = InstrThumb16::generate_decode_table();

        use std::io::prelude::*;
        let mut file = std::fs::File::create("decode_table_new.rs").unwrap();

        for (idx, item) in self.dct.iter().enumerate() {
            writeln!(file, "{:#06x} // {:?} {:#018b}", idx, item, idx).unwrap();
        }
    }

    fn fetch(&self) -> InstrThumb16 {
        unimplemented!()
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }

    /// Steps to execute an instruction
    /// 
    /// -> Fetch 16 bit instruction from program memory, pointed to by R15
    ///     -> Decode 16 bit instruction via DCT lookup
    ///         -> Match instruction to appropriate execution branch
    ///             -> IF Thumb2 extended instruction, fetch second part of instruction
    ///                 -> Decode 32 bit instruction via tree search
    ///                     -> Execute 32 bit instruction
    ///             -> IF standard Thumb16 instruction, execute instruction
    /// 
    fn fde_loop(&mut self) {
        // Core execution loop
        loop {
            unimplemented!();
        }
    }
}
