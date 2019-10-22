#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct OperandData {
    width: u8,
    shift: u8,
    default: Option<u8>,
}

struct InstrData(Vec<OperandData>);

macro_rules! instruction {
    // Entry-point
    { name: $name:ident, $($tail:tt)* } => {
        let _trace = "entry-point";
        let _instruction_name = $name;
        instruction!(@internal $name, $($tail)*);
    };

    // Process an encoding
    { @internal $name:ident, encoding: $($tail:tt)* } => {
        {
            let _trace = "process an encoding";
            instruction!(@internal $name, $($tail)*);
        }
    };

    // Munch base value
    { @internal $name:ident, base: $base:expr, $($tail:tt)* } => {
        let _trace = "munch base value";
        let _base_value = $base;
        instruction!(@internal $name, $base, $($tail)*);
    };
    
    // Munch one operand
    { @internal $name:ident, $base:expr, operand: [$op_name:ident], $($tail:tt)* } => {
        let _trace = "munch one operand";
        let _op_name = stringify!($op_name);        
        instruction!(@internal $name, $base, $($tail)*);
    };

    // Munch last operand
    { @internal $name:ident, $base:expr, operand: [$op_name:ident] $($tail:tt)* } => {
        let _trace = "munch last operand";
        let _op_name = stringify!($op_name);
        instruction!(@internal $name, $($tail)*);
    };
    
    
}

pub fn test_instruction_macro() {
    instruction! {
        name: TestInstruction,
        encoding:
            base: 0x0A0A,
            operand: [foo],
            operand: [bar]

        encoding:
            base: 0xDD99,
            operand: [foo]
    }
}

//macro_rules! instr2 {
//    () => ();
//
//    // Process one operand
//    (
//        @internal
//        $instr_name:ident,
//        $decoded_set:expr,
//        $base:expr,
//        operand: [ name: $op_name:ident, width: $op_width:expr, lsh: $op_shift:expr, default: $op_default:expr ], $($tail:tt)*
//    ) => {
//        {
//            println!("Process one operand");
//            let __op_data = OperandData{ width: $op_width, shift: $op_shift, default: $op_default };
//            for $op_name in 0..::std::cmp::max(1u16, 2u16.pow($op_width)) {
//                instr2!( @internal $instr_name, $decoded_set, $base $($tail)* [$op_name, __op_data])
//            }
//        }
//    };
//
//    // Process last operand
//    (
//        @internal
//        $instr_name:ident,
//        $decoded_set:expr,
//        $base:expr,
//        operand: [ name: $op_name:ident, width: $op_width:expr, lsh: $op_shift:expr, default: $op_default:expr ] $($tail:tt)*
//    ) => {
//        {
//            println!("Process last operand");
//            let __op_data = OperandData{ width: $op_width, shift: $op_shift, default: $op_default };
//            for $op_name in 0..::std::cmp::max(1u16, 2u16.pow($op_width)) {
//                instr2!( @internal $instr_name, $decoded_set, $base, $($tail)* )
//            }
//            instr2!( @internal $instr_name, $decoded_set, $($tail)* )
//
//        }
//    };
//
//    // Construct instruction generator
//    (
//        @internal
//        $instr_name:ident,
//        $decoded_set:expr,
//        $base:expr,
//        $([$op_name:ident, $op_data:expr])* $($tail:tt)*
//    ) => {
//        let hw = $base $( | ($op_data.name << $op_data.shift) )*
//        $decoded_set.push(
//            hw,
//            InstrThumb16::$instr_name{
//                $(
//                    $op_name: ($op_data.default.unwrap_op($op_name) as u8),
//                )*
//            }
//        );
//    };
//
//    // Process one encoding
//    (
//        @internal
//        $instr_name:ident,
//        $decoded_set:expr,
//        encoding: $($tail:tt)*
//    ) => {
//        instr2!( @internal $instr_name, $decoded_set );
//    };
//
//    // Process one encoding inner
//    (
//        @internal
//        $instr_name:ident,
//        $decoded_set:expr,
//        base: $base:expr,
//    ) => {
//        {
//            $decoded_set.append(&mut instr2!( @internal $instr_name, $decoded_set, $base,  $($tail)* ));
//            instr2!( @internal $instr_name, $decoded_set, $($tail)* )
//
//            $decoded_set
//        }
//    };
//
//    // Process an instruction
//    {
//        name: $name:ident,
//        $($tail:tt)*
//    } => {
//        {
//            let mut __decoded_set: Vec<OperandData> = Vec::new();
//            instr2!( @internal $name, __decoded_set, $($tail)* );
//        }
//    };
//}
//
//
//pub fn test_instr2() -> Vec::<(u16, InstrThumb16)> {
//    //instr2! {
//    //    name: ADDimm,
//    //    encoding:
//    //        base: u16,
//    //        operand: [ name: rd, width: 3, lsh: 0, default: None ],
//    //        operand: [ name: rdn, width: 3, lsh: 3, default: None ],
//    //        operand: [ name: imm, width: 3, lsh: 6, default: None ]
//    //    encoding:
//    //        base: 0x3000,
//    //        operand: [ name: imm, width: 8, lsh: 0, default: None ],
//    //        operand: [ name: rdn, width: 3, lsh: 8, default: None ],
//    //        operand: [ name: rd, width: 0, lsh: 0, default: Some(0xFF) ]
//    //};
//
//    instr2! {
//        name: TEST_ONE,
//        encoding:
//            base: u16,
//            operand: [ name: t1, width: 8, lsh: 0, default: None ]
//    }
//
//    Vec::new()
//}



#[derive(Clone, Copy, Debug)]
pub enum InstrThumb16 {
    // ADC
    ADCreg { rm: u8, rdn: u8 }, // 0x4140

    // ADD
    ADDimm { imm: u8, rdn: u8, rd: u8 }, // 0x1C00, 0x3000; encoding: rd == 0xFF ? T2 : T1
    ADDreg { rm: u8, rdn: u8, rd: u8 }, // 0x1800, 0x4400; encoding: rd == 0xFF ? T2 : T1
    ADDspimm { imm: u8, rd: u8}, // 0xA800, 0xB000; encoding: rd == 0xFF ? T2 : T1
    ADDspreg { rm: u8, rdm: u8 }, // 0x4468, 0x4485; encoding: rm == 0x0D ? T1 : T2

    TestInstruction { test_operand: u8 },

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

