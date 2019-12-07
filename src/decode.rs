use crate::instructions::InstrThumb16;
use crate::instructions::NUM_TH16_INSTRUCTIONS;


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



/// What do we need to capture to fully describe an instruction?
///     * Name, long name, and optional description
///     * Family, thumb or thumb2
///     * Invariant, the value which defines the instruction as being itself
///     * The algebraic variant of the instruction
///     * Each individual operand
///     * Individual encodings
/// 
/// What do we need to capture to fully describe an operand?
///     * Name, optional long name, and optional description
///     * Operand arity, or the number of operands
///     * The bit width of each operand
///     * The number of bits shifted from the right to the left of each operand
///     * The language representation of each operand, including whether it is signed or unsigned
///     * Whether the operand is a composite of bit sub-slices of the instruction
/// 
/// How do we want to represent that captured data?
///     * Verbose, immutable, structured data
/// 
/// How can we simplify the definitions of operands?
///     * Templates for commonly used operands?
/// 




// do it all programmatically, abort on general purpose macros. use builder pattern. more verbose but much more sound and more expressive

macro_rules! map_operand {
    ($instr:path, $op:ident, $repr:ident) => {
        #[allow(dead_code)]
        {
            Box::new(|_i, _o| {
                match _i {
                    $instr{ mut $op, .. } => {
                        $op = *_o.downcast_ref::<$repr>().expect("invalid operand downcast")
                    },
                    _ => panic!("invalid operand map")
                }
            })
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_description_builder() {
        let description = InstrDesc::new()
            .name("Branch")
            .desc("Conditional and unconditional branching")
            .encoding(Encoding::new()
                .name("E1")
                .desc("Performs a conditional branch")
                .invariant(0xD000)
                .ctor(Box::new(|| InstrThumb16::BranchE1{ cond: 0u8, imm: 0i8 }))
                .operand(Operand::new()
                    .name("cond")
                    .width(4)
                    .shift(8)
                    .signed(false)
                    .map(map_operand!(InstrThumb16::BranchE1, cond, u8))
                    .build())
                .operand(Operand::new()
                    .name("imm")
                    .width(8)
                    .signed(true)
                    .map(map_operand!(InstrThumb16::BranchE1, imm, i8))
                    .build())
                .build())
            .encoding(Encoding::new()
                .name("E2")
                .desc("Performs an unconditional branch")
                .invariant(0xE000)
                .ctor(Box::new(|| InstrThumb16::BranchE2{ imm: 0i16 }))
                .operand(Operand::new()
                    .name("imm")
                    .width(11)
                    .signed(true)
                    .map(map_operand!(InstrThumb16::BranchE2, imm, i16))
                    .build())
                .build())
            .build();
        println!("{:#?}", description);
    }
}

#[derive(Debug)]
pub struct InstrDesc {
    name: String,
    desc: String,
    encodings: Vec<Encoding>,
}

impl InstrDesc {
    pub fn new() -> InstrDescBuilder {
        InstrDescBuilder {
            inner: InstrDesc {
                name: Default::default(),
                desc: Default::default(),
                encodings: Vec::new()
            }
        }
    }
}

#[derive(Debug)]
pub struct InstrDescBuilder {
    inner: InstrDesc
}

impl InstrDescBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.inner.name = String::from(name);
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        self.inner.desc = String::from(desc);
        self
    }

    pub fn encoding(mut self, encoding: Encoding) -> Self {
        self.inner.encodings.push(encoding);
        self
    }

    pub fn build(self) -> InstrDesc {
        self.inner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrFamily {
    Thumb16,
    Thumb32
}

#[derive(Debug)]
pub struct Encoding {
    name: String,
    desc: String,
    invariant: usize,
    operands: Vec<Operand>,
    ctor: Option<VariantCtorFn>
}

impl Encoding {
    pub fn new() -> EncodingBuilder {
        EncodingBuilder {
            inner: Encoding {
                name: Default::default(),
                desc: Default::default(),
                invariant: Default::default(),
                operands: Vec::new(),
                ctor: None
            }
        }
    }
}


#[derive(Debug)]
pub struct EncodingBuilder {
    inner: Encoding
}

impl EncodingBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.inner.name = String::from(name);
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        self.inner.desc = String::from(desc);
        self
    }

    pub fn operand(mut self, op: Operand) -> Self {
        self.inner.operands.push(op);
        self
    }
    
    pub fn invariant(mut self, invariant: usize) -> Self {
        self.inner.invariant = invariant;
        self
    }

    pub fn ctor(mut self, ctor_func: VariantCtorFn) -> Self {
        self.inner.ctor = Some(ctor_func);
        self
    }

    pub fn build(self) -> Encoding {
        self.inner
    }
}

#[derive(Debug)]
pub struct Operand {
    name: String,
    width: usize,
    shift: usize,
    signed: bool,
    map: Option<OperatorMapFn>
}

impl Operand {
    pub fn new() -> OperandBuilder {
        OperandBuilder {
            inner: Operand {
                name: Default::default(),
                width: Default::default(),
                shift: Default::default(),
                signed: Default::default(),
                map: None
            }
        }
    }
}

#[derive(Debug)]
pub struct OperandBuilder {
    inner: Operand
}

use std::any::Any;

impl OperandBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.inner.name = String::from(name);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.inner.width = width;
        self
    }

    pub fn shift(mut self, shift: usize) -> Self {
        self.inner.shift = shift;
        self
    }

    pub fn signed(mut self, signed: bool) -> Self {
        self.inner.signed = signed;
        self
    }

    pub fn map(mut self, map_func: OperatorMapFn) -> Self {
        self.inner.map = Some(map_func);
        self
    }

    pub fn build(self) -> Operand {
        self.inner
    }
}



pub trait OpMap: Fn(&mut InstrThumb16, &dyn Any) { }
impl<F> OpMap for F where F: Fn(&mut InstrThumb16, &dyn Any) { }

pub type OperatorMapFn = Box<dyn OpMap>;
impl std::fmt::Debug for OperatorMapFn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Operand Mapping Function]")
    }
}

pub trait VariantCtor: Fn() -> InstrThumb16 { }
impl<F> VariantCtor for F where F: Fn() -> InstrThumb16 { }

pub type VariantCtorFn = Box<dyn VariantCtor>;
impl std::fmt::Debug for VariantCtorFn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Variant Constructor Function]")
    }
}
