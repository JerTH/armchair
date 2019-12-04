use crate::instructions::NUM_TH16_INSTRUCTIONS;

#[derive(Debug, Clone, Copy)]
pub struct OperandData {
    pub width: u8,
    pub shift: u8,
    pub default: Option<u8>,
}


/// This macro defines a simple DML which expands into code which generates 16-bit ARMv7-M Thumb instruction decode tables
/// 
/// Because the instruction space for half-word instructions is small enough, we are able to directly map from machine code
/// into decoded instructions using a lookup table without performing any decoding logic. We must however still decode
/// 32-bit double half-word instructions.
/// 
/// Parsing within this macro is performed using a technique known as token-tree munching
/// 
macro_rules! instruction {

    // Entry-point
    //
    // Note: 
    //      The "$($comments)*" expansion is used to capture any doc comments applied to each macro
    //      invocation and may be used for printing prettier generated documentation
    { name: $name:ident, $($tail:tt)* } => {
        {
            let mut _decoded_set: Vec::<(u16, crate::instructions::InstrThumb16)> = Vec::new();
            instruction!(@internal [] [] $name, _decoded_set, $($tail)*);
            _decoded_set
        }
    };

    // Process an encoding
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $( encoding: [ $($tail:tt)* ] ),*
    } => {
        {
            $(
                let _trace = stringify!($($tail)*);
                instruction!(@internal [] [] $name, $dset, $($tail)*);
            )*
        }
    };

    // Munch base value
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, base: $base:expr, $($tail:tt)*
    } => {
        let _trace = stringify!($($tail)*);
        instruction!(@internal [] [] $name, $dset, $base, $($tail)*);
    };

    // Munch base value without operands
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, base: $base:tt $($tail:tt)*
    } => {
        let _trace = stringify!($($tail)*);
        instruction!(@internal [] [] $name, $dset, $base, $($tail)*);
    };
    
    // Munch one operand
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, operand: [$op_name:ident, $op_width:tt << $op_shift:expr], $($tail:tt)*
    } => {
        let _iterations = ::std::cmp::max(1, 2u16.pow($op_width));
        for $op_name in 0.._iterations {    
            let _operator = crate::decode::OperandData {
                width: $op_width,
                shift: $op_shift,
                default: None,
            };

            let _trace = stringify!($($tail)*);
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
            let _operator = crate::decode::OperandData {
                width: $op_width,
                shift: $op_shift,
                default: None,
            };

            let _trace = stringify!($($tail)*);
            instruction!(@internal [$($names,)* $op_name] [$($data,)* _operator] $name, $dset, $base, $($tail)*);
        }
    };

    // Munch last unused operand
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, operand: [$op_name:ident, unused] $($tail:tt)*
    } => {
        let _iterations = ::std::cmp::max(1, 2u16.pow(0));
        for $op_name in 0.._iterations {
            let _operator = crate::decode::OperandData { width: 0, shift: 0, default: Some(0xFF) };

            let _trace = stringify!($($tail)*);
            instruction!(@internal [$($names,)* $op_name] [$($data,)* _operator] $name, $dset, $base, $($tail)*);
        }
    };

    // Terminal
    {
        @internal [$($names:ident),*] [$($data:expr),*]
        $name:ident, $dset:tt, $base:expr, $($tail:tt)*
    } => {
        let _trace = stringify!($base);
        let hw = $base $(| ($names << $data.shift))*;

        let _terminal_trace = stringify!($($tail)*);
        &mut $dset.push(
            (
                hw,
                crate::instructions::InstrThumb16::$name {
                    $(
                        $names: $data.default.unwrap_or($names as u8)
                    ),*
                }
            )
        );
    };
}

macro_rules! define_instructions {
    ($($inst:expr),*) => {
        {
            use crate::instructions::InstrThumb16;

            //let mut __set: Vec<(u16, crate::instructions::InstrThumb16)> = Vec::new();
            let mut __set: [InstrThumb16; NUM_TH16_INSTRUCTIONS] = [InstrThumb16::Undefined; NUM_TH16_INSTRUCTIONS];
            $(
                {
                    let mut __temp = $inst;
                    for item in __temp {
                        __set[item.0 as usize] = item.1;
                    }
                }
            )*
            __set
        }
    };
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


pub fn test_instruction_macro() -> [crate::instructions::InstrThumb16; NUM_TH16_INSTRUCTIONS] {
    define_instructions! {
        instruction! {
            name: AddImm,
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
        }
    }
}
