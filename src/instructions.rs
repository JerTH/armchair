
#[derive(Clone, Copy, Debug)]
pub enum InstrThumb16 {
    // ADC
    ADCreg { rm: u8, rdn: u8 },

    // ADD
    ADDimm { imm: u8, rdn: u8, rd: u8 },
    ADDreg { rm: u8, rdn: u8, rd: u8 },
    ADDspimm { imm: u8, rd: u8},
    ADDspreg { rm: u8, rdm: u8 },

    // LDR
    LDRlit { rt: u8, imm: u8},
    LDRBlit { imm: u8, rn: u8, rt: u8 },

    // MISCELLANEOUS
    NOP,
    UDF { imm: u8 },
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
                    operand: [rd, 1 << 7]
                ]
            },

            instruction! {
                name: LDRlit,
                encoding: [
                    base: 0x4800,
                    operand: [rt, 3 << 8],
                    operand: [imm, 8 << 0]
                ]
            },

            instruction! {
                name: LDRBlit,
                encoding: [
                    base: 0x7800,
                    operand: [imm, 5 << 6],
                    operand: [rn, 3 << 3],
                    operand: [rt, 3 << 0]
                ]
            },

            instruction! {
                name: NOP,
                encoding: [
                    base: 0xBF00
                ]
            },

            instruction! {
                name: UDF,
                encoding: [
                    base: 0xDE00,
                    operand: [imm, 8 << 0],
                ]
            }
        }
    }
}
