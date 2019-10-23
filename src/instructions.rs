
#[derive(Clone, Copy, Debug)]
pub enum InstrThumb16 {
    // ADC
    ADCreg { rm: u8, rdn: u8 }, // 0x4140

    // ADD
    ADDimm { imm: u8, rdn: u8, rd: u8 }, // 0x1C00, 0x3000; encoding: rd == 0xFF ? T2 : T1
    ADDreg { rm: u8, rdn: u8, rd: u8 }, // 0x1800, 0x4400; encoding: rd == 0xFF ? T2 : T1
    ADDspimm { imm: u8, rd: u8}, // 0xA800, 0xB000; encoding: rd == 0xFF ? T2 : T1
    ADDspreg { rm: u8, rdm: u8 }, // 0x4468, 0x4485; encoding: rm == 0x0D ? T1 : T2

    // MISCELLANEOUS
    UNDEFINED,
}

impl InstrThumb16 {
    pub fn generate_instruction_table() -> [InstrThumb16; ::std::u16::MAX as usize] {
        define_instructions! {
            instruction! {
                name: ADDimm,
                encoding:
                    base: 0x1C00,
                    operand: [rd, 3 << 0],
                    operand: [rdn, 3 << 3],
                    operand: [imm, 3 << 6]
            }
        }
    }
}
