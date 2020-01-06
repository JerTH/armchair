#[macro_use]
use std::ops::{ Add, RangeInclusive };
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
///     * Arity, discovered implicitly
///     * Invariant, the value which defines the instruction as being itself
///     * The algebraic variant of the instruction
///     * A description of each operand
///     * Individual encodings
/// 
/// What do we need to capture to fully describe an operand?
///     * Name, optional long name, and optional description
///     * The bit width of the operand
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
                //trace!("_i: {:?}", _i);
                //trace!("    Operand {{ name: {:?}, repr: {:?} }}", _s.name, _s.repr);
                match _i {
                    $instr{ ref mut $op, .. } => {
                        let _temp_downcasted = *_o.downcast_ref::<i64>().expect("invalid signed operand intermediary downcast");
                        *$op = _temp_downcasted as $repr;
                    },
                    m => {
                        panic!(format!("invalid instruction operand field map: {:?}", m));
                    }
                }
            })
        }
    };
}

//#[allow(unused_macros)]
//macro_rules! map_operand {
//    ($instr:path, $op:ident, $repr:ident) => {
//        #[allow(unused_variables)]
//        {
//            Box::new(|_s, mut _i, _o| {
//                trace!("Mapping field...");
//                trace!("    {:?}", _i);
//                trace!("    Operand {{ name: {:?}, repr: {:?} }}", _s.name, _s.repr);
//                trace!("    Prior state: {:?}", _i);
//                match _i {
//                    $instr{ ref mut $op, .. } => {
//                        trace!("    Performing operand downcast");
//                        match _s.repr {
//                            OperandRepr::SignedByte |
//                            OperandRepr::SignedShort |
//                            OperandRepr::SignedWord => {
//                                let _temp = *_o.downcast_ref::<i64>().expect("invalid signed operand intermediary downcast");
//                                *$op = _temp as $repr;
//                            },
//
//                            OperandRepr::UnsignedByte |
//                            OperandRepr::UnsignedShort |
//                            OperandRepr::UnsignedWord => {
//                                let _temp = *_o.downcast_ref::<u64>().expect("invalid unsigned operand intermediary downcast");
//                                *$op = _temp as $repr;
//                            },
//                        }
//                        
//                    },
//                    m => {
//                        panic!("invalid instruction operand field map: {:?}", m);
//                    }
//                }
//                trace!("    Resulting state: {:?}", _i);
//            })
//        }
//    };
//}

#[cfg(test)]
mod test {
    use super::*;

    fn test_env() {
        // Setup the test environment to print trace logs by default
        let log_filter = "debug";
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

        let branch = InstrDesc::new()
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

        let test_desc = InstrDesc::new()
            .name("SimpleTest")
            .desc("Simple unit testing instruction")
            .encoding(Encoding::new()
                .name("E1")
                .desc("First Encoding")
                .invariant(0xF000)
                .ctor(Box::new(|| InstrThumb16::Test{ a: 0u8, b: 0, c: 0u8} ))
                .operand(Operand::new()
                    .name("a")
                    .width(2)
                    .shift(6)
                    .repr(OperandRepr::UnsignedByte)
                    .map(map_operand!(InstrThumb16::Test, a, u8))
                    .build())
                .operand(Operand::new()
                    .name("b")
                    .width(2)
                    .shift(3)
                    .repr(OperandRepr::UnsignedByte)
                    .map(map_operand!(InstrThumb16::Test, b, u8))
                    .build())
                .operand(Operand::new()
                    .name("c")
                    .width(2)
                    .shift(0)
                    .repr(OperandRepr::UnsignedByte)
                    .map(map_operand!(InstrThumb16::Test, c, u8))
                    .build())
                .build())
            .build();

        trace!("{:#?}", branch);
        
        for enc in branch.encodings {
            enc.generate_decode_table();
        }

        for enc in test_desc.encodings {
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
        trace!("New instruction description");

        InstrDescBuilder {
            name: None,
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
    name: Option<String>,
    inner: InstrDesc
}

impl InstrDescBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        self.inner.desc = String::from(desc);
        self
    }
    
    pub fn encoding(mut self, encoding: Encoding) -> Self {
        trace!("Adding instruction encoding");

        // Insert the parent instruction name to the encoding for debug printing purposes
        let mut temp = encoding;
        if let Some(name) = &self.name {
            temp.parent = name.clone();
        } else {
            panic!("Instructions must be named before encodings can be added");
        }

        self.inner.encodings.push(temp);
        self
    }

    pub fn build(mut self) -> InstrDesc {
        self.inner.name = self.name.expect("Attempted to build an unnamed instruction");

        debug!("Building instruction description {:?} with {:?} encoding(s)", self.inner.name, self.inner.encodings.len());
        self.inner
    }
}

#[derive(Debug)]
pub struct Encoding {
    parent: String,
    name: String,
    desc: String,
    invariant: usize,
    operands: Vec<Operand>,
    ctor: Option<VariantCtorFn>
}

impl Encoding {
    pub fn new() -> EncodingBuilder {
        trace!("New encoding...");
        
        EncodingBuilder {
            inner: Encoding {
                parent: Default::default(),
                name: Default::default(),
                desc: Default::default(),
                invariant: Default::default(),
                operands: Vec::new(),
                ctor: None
            }
        }
    }

    pub fn arity(&self) -> usize {
        self.operands.len()
    }

    pub fn generate_decode_table(&self) {
        let rdb = RecursiveDecoderBuilder::new(&self);
        let dct = rdb.build_decode_table();

        for item in dct {
            println!("{:?}", item);
        }
        //let decode_table = self.gdt_recursive(&self.operands, None, 0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionCodecPair {
    encoded: u16,
    decoded: InstrThumb16
}

impl InstructionCodecPair {
    fn new(encoded: u16, decoded: InstrThumb16) -> InstructionCodecPair {
        InstructionCodecPair {
            encoded, decoded
        }
    }
}

pub type DecodeTable = Vec<InstructionCodecPair>;

struct RecursiveDecoderBuilder<'a> {
    encoding: &'a Encoding,

    /// The resulting decode table
    dectab: RefCell<DecodeTable>,

    // this holds the current state of the instruction we are processing
    //state: Rc<RefCell<InstrThumb16>>
}

impl<'a> RecursiveDecoderBuilder<'a> {
    pub fn new(encoding: &'a Encoding) -> RecursiveDecoderBuilder<'a> {
        RecursiveDecoderBuilder {
            encoding: encoding,
            dectab: RefCell::new(Vec::new()),
        }
    }

    pub fn build_decode_table(self) -> DecodeTable {
        let variant_constructor = self.encoding.ctor.as_ref().expect("no variant ctor");
        let variant = (*variant_constructor)();

        let mut table = Vec::new();
        let mut state = variant;
        self.build_decode_table_recursive(0, &mut state, &mut table);

        for (i, item) in table.iter().enumerate() {
            //println!("{:04}: {:?}", i + 1, item);
        }

        self.dectab.into_inner()
    }
    
    fn build_decode_table_recursive(&self, idx: usize, state: &mut InstrThumb16, output: &mut Vec<InstrThumb16>) {
        trace!("Building decode table from instruction operand set (recursion level: {})", idx);
        
        // Map all permutations of a given instruction into a list of decoded instructions
        
        // If we have an operand
        if let Some(operand) = self.encoding.operands.get(idx) {
            let field_map = operand.map.as_ref().unwrap();

            // Calculate the permutation range of the operand
            let (low, high) = operand.permutations();

            // For each permutation
            for p in RangeInclusive::new(low, high) {

                // Apply the field mapping function to update the value of the state
                (*field_map)(&operand, state, &p);
                
                // try and go one level deeper
                self.build_decode_table_recursive(idx.add(1), state, output);                
            }
        } else {
            // Terminal, copy our current state into the decode table as a legal permutation
            output.push(state.clone());
        }
        
        // if we are out of the recursion stack and about to return
        if idx == 0 {
            debug!("Built decode table for encoding {:?} of {:?} with {} permutation(s)", self.encoding.name, self.encoding.parent, output.len());
        }
    }

    #[allow(dead_code)]
    fn build_decoder(&self) {
        unimplemented!()
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

    pub fn operand(mut self, op: Operand) -> Self {
        trace!("Adding encoding operand {:?}, width {:?}, shifted {:?} bits to the left", op.name, op.width, op.shift);

        self.inner.operands.push(op);
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
        trace!("Building encoding {:?} with {:?} operands", self.inner.name, self.inner.operands.len());

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
    
    pub fn permutations(&self) -> (i64, i64) {
        let range = std::cmp::max(1u64, 2u64.pow(self.width as u32));
        match self.repr {
            OperandRepr::SignedByte |
            OperandRepr::SignedShort |
            OperandRepr::SignedWord => {
                let low = ((range / 2) as i64) * -1;
                let high = ((range / 2) - 1) as i64;
                (low, high)
            },

            OperandRepr::UnsignedByte |
            OperandRepr::UnsignedShort |
            OperandRepr::UnsignedWord => {
                (0i64, (range - 1) as i64)
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
        trace!("Building operand {:?}", self.inner.name);

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
