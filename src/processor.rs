use crate::instructions;
use crate::instructions::{ InstrThumb16 };
use crate::memory::{ Register, RegisterBank, Memory };
use crate::loader::ProgramImage;

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
    reset: usize,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            dct: [InstrThumb16::Undefined; instructions::NUM_TH16_INSTRUCTIONS],
            reg: RegisterBank::new(),
            mem: Memory::alloc(0),
            reset: 0,
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

    pub fn load(&mut self, image: ProgramImage) {
        self.reset = image.entry();
        self.mem = image.into_raw_image();
    }

    pub fn reset(&mut self) {
        self.reg[Register::PC] = self.reset as u32 - 1;
    }

    pub fn run(&mut self) {
        self.fde_loop()
    }
    
    fn fetch(&self) -> u16 {
        let at = self.reg[Register::PC] as usize;
        self.mem.read_u16(at)
    }

    fn decode(&self, instruction: u16) -> InstrThumb16 {
        let decoded = self.dct[instruction as usize];
        decoded
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
        let mut cycles = 0;
        let debug_cycle_limit = 10;

        println!("Beginning execution...");

        // Core execution loop
        loop {
            print!("[PC: {:06X}] ", self.reg[Register::PC]);
            let fetched = self.fetch();
            let decoded = self.decode(fetched);

            print!("{:016b} {:04X} ", fetched, fetched);

            match decoded {
                InstrThumb16::BranchE1{ cond, imm } => {
                    let target = (imm as i8) as i32;
                    print!("exec branch e1: [cond, target] = [{:04X}, {:#06X}] ({}:{})", cond, target, cond, imm);
                    self.reg[Register::PC] = ((self.reg[Register::PC] as i32) + target) as u32;
                },

                u => {
                    print!("unhandled instruction: {:?}", u)
                }
            }

            print!("\n");

            cycles = cycles + 1;
            if cycles >= debug_cycle_limit {
                break;
            }
        }
    }
}
