use crate::instructions::{ InstrThumb16 };
use crate::memory::{ Register, RegisterBank, Memory };

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
/// 
pub struct Processor {
    dct: [InstrThumb16; ::std::u16::MAX as usize],
    registers: RegisterBank,
    memory: Memory,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            dct: [InstrThumb16::UNDEFINED; ::std::u16::MAX as usize],
            registers: RegisterBank::new(),
            memory: Memory::new()
        }
    }

    pub fn init(&mut self) {
        self.dct = InstrThumb16::generate_decode_table();
    }

    fn fetch_and_decode(&self) -> InstrThumb16 {
        unimplemented!()
    }

    pub fn run(&mut self) -> std::result::Result<(), ()> {
        let mut result: std::result::Result<(), ()> = Ok(());

        // FDE Loop
        loop {
            let instruction = self.fetch_and_decode();
            
            match instruction {
                _ => { unimplemented!("{:?}", instruction) }
            }
        }
    }
}
