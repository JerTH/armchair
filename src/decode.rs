#![allow(dead_code)]

macro_rules! instr2_16 {
    {
        @terminal $name:ident, $dset:ident, $base:expr,
        $AA_name:ident, $AA_width:expr, $AA_shift:expr, $AA_default:expr,
        $BB_name:ident, $BB_width:expr, $BB_shift:expr, $BB_default:expr,
        $CC_name:ident, $CC_width:expr, $CC_shift:expr, $CC_default:expr
    } => {
        let __it1 = ::std::cmp::max(1, 2u16.pow($AA_width));
        let __it2 = ::std::cmp::max(1, 2u16.pow($BB_width));
        let __it3 = ::std::cmp::max(1, 2u16.pow($CC_width));
        for $AA_name in 0..__it1 {
            for $BB_name in 0..__it2 {
                for $CC_name in 0..__it3 {
                    let hw = $base
                        | ($AA_name << $AA_shift)
                        | ($BB_name << $BB_shift)
                        | ($CC_name << $CC_shift);
                    $dset.push((
                        hw,
                        InstrThumb16::$name{
                            $AA_name: ($AA_default.unwrap_or($AA_name)) as u8,
                            $BB_name: ($BB_default.unwrap_or($BB_name)) as u8,
                            $CC_name: ($CC_default.unwrap_or($CC_name)) as u8
                        }
                    ));
                }
            }
        }
    };
}

macro_rules! instr2 {
    // First expansion
    {
        name: $name:ident,
        $($tail:tt)*
    } => {
        {
            let mut __decoded_set: Vec::<(u16, InstrThumb16)> = Vec::new();
            instr2!{ @recurse $name, __decoded_set, $($tail)* }
            __decoded_set
        }
    };

    // Recursive 16-bit encoding expansion
    {
        @recurse
        $name:ident,
        $dset:ident,
        encoding16: {
            base: $base:expr,
            operand: { $AA_name:ident; $AA_width:expr; $AA_shift:expr; $AA_default:expr },
            operand: { $BB_name:ident; $BB_width:expr; $BB_shift:expr; $BB_default:expr },
            operand: { $CC_name:ident; $CC_width:expr; $CC_shift:expr; $CC_default:expr }
        },
        $($tail:tt)*
    } => {
        instr2_16!{
            @terminal $name, $dset, $base,
            $AA_name, $AA_width, $AA_shift, $AA_default,
            $BB_name, $BB_width, $BB_shift, $BB_default,
            $CC_name, $CC_width, $CC_shift, $CC_default
        }
        instr2!{ @recurse $name, $dset, $($tail)* }
    };

    // Terminal 16-bit encoding expansion
    {
        @recurse
        $name:ident,
        $dset:ident,
        encoding16: {
            base: $base:expr,
            operand: { $AA_name:ident; $AA_width:expr; $AA_shift:expr; $AA_default:expr },
            operand: { $BB_name:ident; $BB_width:expr; $BB_shift:expr; $BB_default:expr },
            operand: { $CC_name:ident; $CC_width:expr; $CC_shift:expr; $CC_default:expr }
        }
        $($tail:tt)*
    } => {
        instr2_16!{
            @terminal $name, $dset, $base,
            $AA_name, $AA_width, $AA_shift, $AA_default,
            $BB_name, $BB_width, $BB_shift, $BB_default,
            $CC_name, $CC_width, $CC_shift, $CC_default
        }
    };
}

fn test_instr2() -> Vec::<(u16, InstrThumb16)> {
    instr2!{
        name: ADDimm,
        encoding16: {
            base: 0x1C00,
            operand: { rd; 3; 0; None },
            operand: { rdn; 3; 3; None },
            operand: { imm; 3; 6; None }
        },
        encoding16: {
            base: 0x3000,
            operand: { imm; 8; 0; None },
            operand: { rdn; 3; 8; None },
            operand: { rd; 0; 0; Some(0xFF) }
        }
    }
}

macro_rules! instructions {
    ($($inst:expr),*) => {
        {
            let mut __set: Vec<(u16, InstrThumb16)> = Vec::new();

            $(
                {
                    let mut __temp = $inst;
                    __set.append(&mut __temp);
                }
            )*

            //__set.sort_by(|a, b| a.0.cmp(&b.0) );
            __set
        }
        
    };
}

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

fn generate_decode_table() -> Vec<(u16, InstrThumb16)> {
    instructions!{
        instr2!{
            name: ADDimm,
            encoding16: {
                base: 0x1C00,
                operand: { rd; 3; 0; None },
                operand: { rdn; 3; 3; None },
                operand: { imm; 3; 6; None }
            },
            encoding16: {
                base: 0x3000,
                operand: { imm; 8; 0; None },
                operand: { rdn; 3; 8; None },
                operand: { rd; 0; 0; Some(0xFF) }
            }
        },

        instr2!{
            name: ADDreg,
            encoding16: {
                base: 0x1C00,
                operand: { rd; 3; 0; None },
                operand: { rdn; 3; 3; None },
                operand: { rm; 3; 6; None }
            },
            encoding16: {
                base: 0x3000,
                operand: { rdn; 3; 0; None },
                operand: { rm; 4; 3; None },
                operand: { rd; 1; 7; Some(0xFF) }
            }
        }
    }
}

pub fn generate_test() -> Vec<(u16, InstrThumb16)> {
    let mut decode_table: Vec<(u16, InstrThumb16)> = Vec::new();
    decode_table.append(&mut generate_decode_table());
    decode_table
}



/* 
 * ARMv7-M THUMB ENCODING
 * 
 * The Thumb instruction stream is a sequence of halfword-aligned halfwords. Each Thumb instruction is either a
 * single 16-bit halfword in that stream, or a 32-bit instruction consisting of two consecutive halfwords in that stream.
 * 
 * If bits [15:11] of the halfword being decoded take any of the following values, the halfword is the first halfword of
 * a 32-bit instruction:
 *  - 0b11101
 *  - 0b11110
 *  - 0b11111
 * 
 * 
 * 
 * 16 BIT THUMB INSTRUCTION ENCODING
 * =================================================
 * |15 14 13 12 11 10|09 08 07 06 05 04 03 02 01 00|
 * |opcode           |                             |
 * =================================================
 * 
 * 
 * SHIFT (imm) ADD, SUBTRACT, MOVE, COMPARE
 * =================================================
 * |15 14|13 12 11 10 09|08 07 06 05 04 03 02 01 00|
 * |0  0 |opcode        |                          |
 * =================================================
 * 
 * 
 * DATA PROCESSING INSTRUCTION ENCODING
 * =================================================
 * |15 14 13 12 11 10|09 08 07 06|05 04 03 02 01 00|
 * |0  1  0  0  0  0 |opcode     |                 |
 * =================================================
 * 
 * 
 * SPECIAL DATA INSTRUCTIONS AND BRANCH AND EXCHANGE
 * =================================================
 * |15 14 13 12 11 10|09 08 07 06|05 04 03 02 01 00|
 * |0  1  0  0  0  1 |opcode     |                 |
 * =================================================
 * 
 * 
 * LOAD/STORE SINGLE DATA ITEM
 * =================================================
 * |15 14 13 12|11 10 09|08 07 06 05 04 03 02 01 00|
 * |opA        |opB     |                          |
 * =================================================
 * 
 * NOTE:
 *  These instructions have one of the following values in opA:
 *   - 0b0101
 *   - 0b011x
 *   - 0b100x
 * 
 * 
 * MISCELLANEOUS 16-BIT INSTRUCTIONS
 * =================================================
 * |15 14 13 12|11 10 09 08 07 06 05|04 03 02 01 00|
 * |1  0  1  1 |opcode              |              |
 * =================================================
 * 
 * 
 * IF/THEN AND HINTS
 * =================================================
 * |15 14 13 12|11 10 09 08|07 06 05 04|03 02 01 00|
 * |1  0  1  1 |1  1  1  1 |opA        |opB        |
 * =================================================
 * 
 * NOTE:
 *  Other encodings in this space are unallocated hints. They execute as NOPs, but software must not use them.
 * 
 * 
 * CONDITIONAL BRANCH AND SUPERVISOR CALL
 * =================================================
 * |15 14 13 12|11 10 09 08|07 06 05 04 03 02 01 00|
 * |1  1  0  1 |opcode     |                       |
 * =================================================
 * 
 * 
 * 32-BIT THUMB INSTRUCTION ENCODING
 * =================================================================================================
 * |15 14 13|12 11|10 09 08 07 06 05 04|03 02 01 00|15|14 13 12 11 10 09 08 07 06 05 04 03 02 01 00|
 * |1  1  1 |op1  |op2                 |           |op|                                            |
 * =================================================================================================
 * 
 * 
 * 
 */

