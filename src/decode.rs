#[macro_use]
use std::any::{ TypeId, Any };
use std::rc::Rc;
use std::cell::RefCell;
use log::{ info, debug, trace };
use crate::instructions::InstrThumb16;


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


#[allow(unused_macros)]
macro_rules! map_operand {
    ($instr:path, $op:ident, $repr:ident) => {
        #[allow(unused_variables)]
        {
            Box::new(|_s, mut _i, _o| {
                trace!("Mapping field...");
                trace!("    {:?}", _i);
                trace!("    Operand {{ name: {:?}, repr: {:?} }}", _s.name, _s.repr);
                trace!("    Prior state: {:?}", _i);
                match _i {
                    $instr{ ref mut $op, .. } => {
                        trace!("    Performing operand downcast");
                        match _s.repr {
                            OperandRepr::SignedByte |
                            OperandRepr::SignedShort |
                            OperandRepr::SignedWord => {
                                let _temp = *_o.downcast_ref::<i64>().expect("invalid operand intermediary downcast");
                                *$op = _temp as $repr;
                            },

                            OperandRepr::UnsignedByte |
                            OperandRepr::UnsignedShort |
                            OperandRepr::UnsignedWord => {
                                let _temp = *_o.downcast_ref::<u64>().expect("invalid operand intermediary downcast");
                                *$op = _temp as $repr;
                            },
                        }
                        
                    },
                    _ => panic!("invalid instruction operand field map")
                }
                trace!("    Resulting state: {:?}", _i);
            })
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_env() {
        let log_filter = "trace";
        let write_style = "always";

        let env = env_logger::Env::default()
            .default_filter_or(log_filter)
            .default_write_style_or(write_style);

        let mut builder = env_logger::from_env(env);
        builder.is_test(true);
        builder.init();

        // needed to avoid jumbled output on first line of test stdout
        println!();
    }
    
    #[test]
    fn instruction_description_builder() {
        test_env();

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
                    .repr(OperandRepr::UnsignedByte)
                    .map(map_operand!(InstrThumb16::BranchE1, cond, u8))
                    .build())
                .operand(Operand::new()
                    .name("imm")
                    .width(8)
                    .repr(OperandRepr::SignedByte)
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
                    .repr(OperandRepr::SignedShort)
                    .map(map_operand!(InstrThumb16::BranchE2, imm, i16))
                    .build())
                .build())
            .build();

        for enc in description.encodings {
            enc.generate_decode_table();
        }
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
        trace!("New instruction description...");

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
        trace!("Setting instruction name: {:?}", name);

        self.inner.name = String::from(name);
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        trace!("Setting instruction description: {:?}", desc);

        self.inner.desc = String::from(desc);
        self
    }

    pub fn encoding(mut self, encoding: Encoding) -> Self {
        trace!("Adding instruction encoding...");

        self.inner.encodings.push(encoding);
        self
    }

    pub fn build(self) -> InstrDesc {
        info!("Building instruction description {:?} with {:?} encodings", self.inner.name, self.inner.encodings.len());
        self.inner
    }
}

#[derive(Debug)]
pub struct Encoding {
    name: String,
    desc: String,
    invariant: usize,
    operands: Rc<RefCell<Vec<Operand>>>,
    ctor: Option<VariantCtorFn>
}

impl Encoding {
    pub fn new() -> EncodingBuilder {
        trace!("New encoding...");
        
        EncodingBuilder {
            inner: Encoding {
                name: Default::default(),
                desc: Default::default(),
                invariant: Default::default(),
                operands: Rc::new(RefCell::new(Vec::new())),
                ctor: None
            }
        }
    }

    pub fn generate_decode_table(&self) {
        // recursive calls on operands to capture full operand depth
        let ops = self.operands.clone();
        let dct = self.gdt_recursive(ops, None);
    }

    fn gdt_recursive(&self, ops: Rc<RefCell<Vec<Operand>>>, last: Option<(u16, InstrThumb16)>) {
        trace!("gdt_recursive");

        let mut dt: Vec<(u16, InstrThumb16)> = Vec::new();

        if let Some(op) = self.operands.borrow_mut().pop() {
            trace!("gdt_recursive got op Operand {{ {:?} }}", op.name);

            let ctorf = self.ctor.as_ref().expect("no variant ctor");
            let omapf = op.map.as_ref().expect("no decoded field map");
            let invar = self.invariant;

            if let Some(last) = last {
                trace!("gdt_recursive has last {:?}", last);

                // nested for loops on each operand, 

                // we are one or more levels deep
                
                //dt.append(Encoding::gdt_recursive(operands, dct));
            } else {
                trace!("gdt_recursive first call");

                // we are the first level

                let mut instr = (*ctorf)();
                let iters = ::std::cmp::max(1u64, 2u64.pow(op.width as u32));

                trace!("gdt_recursive created instruction {:?}", instr);
                
                match op.repr {
                    OperandRepr::SignedByte |
                    OperandRepr::SignedShort |
                    OperandRepr::SignedWord => {
                        let low = ((iters / 2) as i64) * -1;
                        let high = (iters / 2) as i64;
                        trace!("gdt_recursive field iter range {:?}:{:?}", low, high);

                        for i in low..high {
                            // set our operand to the current iteration indice, then pass the state of each loop to a recursive call
                            (*omapf)(&op, &mut instr, &i);
                            trace!("gdt_recursive inner loop, instruction state: {:?}", instr);
        
                            self.gdt_recursive(ops.clone(), Some((0u16, instr)));
                        }
                    },

                    OperandRepr::UnsignedByte |
                    OperandRepr::UnsignedShort |
                    OperandRepr::UnsignedWord => {
                        trace!("gdt_recursive field iter range 0:{:?}", iters);

                        for i in 0..iters {
                            // set our operand to the current iteration indice, then pass the state of each loop to a recursive call
                            (*omapf)(&op, &mut instr, &i);
                            trace!("gdt_recursive inner loop, instruction state: {:?}", instr);
        
                            self.gdt_recursive(ops.clone(), Some((0u16, instr)));
                        }
                    }
                }
            }
        } else {
            // no more operands to process, return the decode table
        }
    }
}

#[derive(Debug)]
pub struct EncodingBuilder {
    inner: Encoding
}

impl EncodingBuilder {
    pub fn name(mut self, name: &str) -> Self {
        trace!("Setting encoding name: {:?}", name);

        self.inner.name = String::from(name);
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        trace!("Setting encoding description: {:?}", desc);

        self.inner.desc = String::from(desc);
        self
    }

    pub fn operand(self, op: Operand) -> Self {
        trace!("Adding encoding operand {:?}, width {:?}, shifted {:?} bits to the left", op.name, op.width, op.shift);

        self.inner.operands.borrow_mut().push(op);
        self
    }
    
    pub fn invariant(mut self, invariant: usize) -> Self {
        trace!("Setting encoding invariant: {:#06X}", invariant);

        self.inner.invariant = invariant;
        self
    }

    pub fn ctor(mut self, ctor_func: VariantCtorFn) -> Self {
        trace!("Setting encoding variant constructor method");

        self.inner.ctor = Some(ctor_func);
        self
    }

    pub fn build(self) -> Encoding {
        debug!("Building instruction encoding {:?} with {:?} operands", self.inner.name, self.inner.operands.borrow_mut().len());

        self.inner
    }
}

#[derive(Debug)]
pub struct Operand {
    name: String,
    width: usize,
    shift: usize,
    typeid: TypeId,
    repr: OperandRepr,
    map: Option<OperatorMapFn>
}

impl Operand {
    pub fn new() -> OperandBuilder {
        OperandBuilder {
            inner: Operand {
                name: Default::default(),
                width: Default::default(),
                shift: Default::default(),
                repr: OperandRepr::UnsignedByte,
                typeid: std::any::TypeId::of::<Self>(),
                map: None
            }
        }
    }
}

#[derive(Debug)]
pub struct OperandBuilder {
    inner: Operand
}

impl OperandBuilder {
    pub fn name(mut self, name: &str) -> Self {
        trace!("Setting operand name {:?}", name);

        self.inner.name = String::from(name);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        trace!("Setting operand width {:?}", width);

        self.inner.width = width;
        self
    }

    pub fn shift(mut self, shift: usize) -> Self {
        trace!("Setting operand shift {:?}", shift);

        self.inner.shift = shift;
        self
    }

    pub fn repr(mut self, repr: OperandRepr) -> Self {
        trace!("Setting operand repr {:?}", repr);

        self.inner.repr = repr;
        self
    }

    pub fn map(mut self, map_func: OperatorMapFn) -> Self {
        trace!("Setting operand to decoded instruction mapping method");

        self.inner.map = Some(map_func);
        self
    }

    pub fn build(self) -> Operand {
        debug!("Building instruction operand {:?}", self.inner.name);

        self.inner
    }
}

#[derive(Debug, Clone)]
pub enum OperandRepr {
    SignedByte,
    SignedShort,
    SignedWord,
    UnsignedByte,
    UnsignedShort,
    UnsignedWord,
}

pub trait OperandMap: Fn(&Operand, &mut InstrThumb16, &dyn Any) { }

impl<F: Clone> OperandMap for F where F: Fn(&Operand, &mut InstrThumb16, &dyn Any) { }

pub type OperatorMapFn = Box<dyn OperandMap>;

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
