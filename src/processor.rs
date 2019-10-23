use crate::instructions::{ InstrThumb16 };

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
/// 
pub struct Processor {
    dct: [InstrThumb16; ::std::u16::MAX as usize],
    registers: [u32; ::std::u8::MAX as usize],
    program: Vec::<u16>,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            dct: [InstrThumb16::UNDEFINED; ::std::u16::MAX as usize],
            registers: [0; ::std::u8::MAX as usize],
            program: Vec::<u16>::new(),
        }
    }

    pub fn init(&mut self) {
        self.dct = InstrThumb16::generate_decode_table();
    }

    pub fn load(&mut self, program: &[u16]) {
        self.program = program.to_vec();
    }

    pub fn run(&mut self) -> std::result::Result<(), ()> {
        let mut result: std::result::Result<(), ()> = Ok(());

        // FDE Loop
        while (self.registers[Register::PC as usize] as usize) < self.program.len() {
            let instruction = self.decode(self.fetch());
            self.registers[Register::PC as usize] += 1;

            match instruction {
                InstrThumb16::NOP => {
                    // Do nothing
                },

                InstrThumb16::UNDEFINED => {
                    result = Err(());
                    break;
                },
                _ => { panic!("Unimplemented instruction: {:?}", instruction) }
            }
        }
        result
    }

    fn fetch(&self) -> u16 {
        let fetched = self.program[self.registers[Register::PC as usize] as usize];

        print!("fetched: {:#06X}", fetched);
        fetched
    }

    fn decode(&self, instruction: u16) -> InstrThumb16 {
        let decoded = self.dct[instruction as usize];
        
        println!(" ... decoded: {:?}", decoded);
        decoded
    }
}

pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 15,
}

impl std::ops::Index<Register> for Processor {
    type Output = u32;
    fn index(&self, idx: Register) -> &Self::Output {
        &self.registers[idx as usize]
    }
}
