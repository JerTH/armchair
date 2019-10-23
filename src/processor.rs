use crate::instructions::{ InstrThumb16 };

pub struct Processor {
    decode_table: [InstrThumb16; ::std::u16::MAX as usize],
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            decode_table: [InstrThumb16::UNDEFINED; ::std::u16::MAX as usize],
        }
    }

    pub fn init(&mut self) {
        self.decode_table = InstrThumb16::generate_instruction_table();
    }
}
