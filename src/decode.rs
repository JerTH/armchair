#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct OperandData {
    width: u8,
    shift: u8,
    default: Option<u8>,
}

macro_rules! instruction {
    // Entry-point
    { name: $name:ident, $($tail:tt)* } => {
        {
            let mut _dset: Vec::<(u16, InstrThumb16)> = Vec::new();
            instruction!(@internal [] [] $name, _dset, $($tail)*);
            _dset
        }
    };

    // Process an encoding
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, encoding: $($tail:tt)*
    } => {
        {
            instruction!(@internal [] [] $name, $dset, $($tail)*);
        }
    };

    // Munch base value
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, base: $base:expr, $($tail:tt)*
    } => {
        instruction!(@internal [] [] $name, $dset, $base, $($tail)*);
    };
    
    // Munch one operand
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, operand: [$op_name:ident, $op_width:tt << $op_shift:expr], $($tail:tt)*
    } => {
        let _iterations = ::std::cmp::max(1, 2u16.pow($op_width));
        for $op_name in 0.._iterations {    
            let _operator = OperandData {
                width: $op_width,
                shift: $op_shift,
                default: None,
            };

            instruction!(@internal [$($names,)* $op_name] [$($data,)* _operator] $name, $dset, $base, $($tail)*);
        }
    };

    // Munch last operand
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, operand: [$op_name:ident, $op_width:tt << $op_shift:expr] $($tail:tt)*
    } => {
        let _iterations = ::std::cmp::max(1, 2u16.pow($op_width));
        for $op_name in 0.._iterations {
            let _operator = OperandData {
                width: $op_width,
                shift: $op_shift,
                default: None,
            };

            instruction!(@internal [$($names,)* $op_name] [$($data,)* _operator] $name, $dset, $base, $($tail)*);
        }
    };

    // Terminal
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, $($tail:tt)*
    } => {
        let hw = $base $(| ($names << $data.shift))*;

        &mut $dset.push(
            (
                hw,
                InstrThumb16::$name {
                    $(
                        $names: ($names as u8)
                    ),*
                }
            )
        );
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


pub fn test_instruction_macro() -> Vec::<(u16, InstrThumb16)> {

    return instruction! {
        name: ADDimm,
        encoding:
            base: 0x1C00,
            operand: [rd, 3 << 0],
            operand: [rdn, 3 << 3],
            operand: [imm, 3 << 6]
    }

}


