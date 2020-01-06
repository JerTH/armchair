//! Data Processing Instruction Definitions
//! 
//! 
//! The instructions defined in this module are listed in the ARMv7-m Architecture Reference Manual on page 131:
//! 
//!   OpCode  Instruction
//!   0000    Bitwise AND               AND (register) on page A7-201
//!   0001    Exclusive OR              EOR (register) on page A7-233
//!   0010    Logical Shift Left        LSL (register) on page A7-283
//!   0011    Logical Shift Right       LSR (register) on page A7-285
//!   0100    Arithmetic Shift Right    ASR (register) on page A7-204
//!   0101    Add with Carry            ADC (register) on page A7-188
//!   0110    Subtract with Carry       SBC (register) on page A7-347
//!   0111    Rotate Right              ROR (register) on page A7-339
//!   1000    Set flags on bitwise AND  TST (register) on page A7-420
//!   1001    Reverse Subtract from 0   RSB (immediate) on page A7-341
//!   1010    Compare Registers         CMP (register) on page A7-224
//!   1011    Compare Negative          CMN (register) on page A7-222
//!   1100    Logical OR                ORR (register) on page A7-310
//!   1101    Multiply Two Registers    MUL (register) on page A7-302
//!   1110    Bit Clear                 BIC (register) on page A7-210
//!   1111    Bitwise NOT               MVN (register) on page A7-304
//! 
//! Data processing instruction encoding:
//! =================================================
//! |15 14 13 12 11 10|09 08 07 06|05 04 03 02 01 00|
//! |0  1  0  0  0  0 |opcode     |                 |
//! =================================================

use crate::instructions::InstrThumb16;

pub struct DataProcessingInstructions {
    dct: Option<Vec<InstrThumb16>>,
}
