
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
    NOP,
    UNDEFINED,
}

impl InstrThumb16 {
    pub fn generate_decode_table() -> [InstrThumb16; ::std::u16::MAX as usize] {


        define_instructions! {

            // ADDimm
            //
            // This instruction adds an immediate value to a register value, and writes the result to the destination register. It can
            // optionally update the condition flags based on the result.
            //
            instruction! {
                name: ADDimm,
                encoding: [
                    base: 0x1C00,
                    operand: [rd, 3 << 0],
                    operand: [rdn, 3 << 3],
                    operand: [imm, 3 << 6]
                ],
                encoding: [
                    base: 0x3000,
                    operand: [imm, 8 << 0],
                    operand: [rdn, 3 << 8],
                    operand: [rd, unused]
                ]
            },

            //  ADDreg
            // 
            // ADD (register) adds a register value and an optionally-shifted register value, and writes the result to the destination
            // register. It can optionally update the condition flags based on the result.
            // 
            instruction! {
                name: ADDreg,
                encoding: [
                    base: 0b0001_1000_0000_0000,
                    operand: [rm, 3 << 6],
                    operand: [rdn, 3 << 3],
                    operand: [rd, 3 << 0]
                ],
                encoding: [
                    base: 0b0100_0100_0000_0000,
                    operand: [rm, 4 << 3],
                    operand: [rdn, 3 << 0],
                    operand: [rd, 1 << 7],
                ]
            },

            instruction! {
                name: NOP,
                encoding: [
                    base: 0xBF00
                ]
            }
        }
    }
}
