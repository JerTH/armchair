// ARMv7M-M Memory Model

use std::ops::{ Index, IndexMut };
 
/// Addressable registers
pub enum Register {
    // Thumb16 addressable
    R0, R1, R2, R3,
    R4, R5, R6, R7,
    
    // Thumb32 addressable
    R8, R9, R10, R11, R12,

    // Stack Pointer
    SPM,
    SPP,

    // Link Register
    LR,

    // Program Counter
    //   PC is defined to be the address of the current instruction.
    //   The offset of 4 bytes is applied to it by the register access functions.
    PC,
    
    // Program status registers
    APSR,
    IPSR,
    EPSR,
}

pub struct RegisterBank {
    registers: [u32; ::std::u8::MAX as usize],
}

impl RegisterBank {
    pub fn new() -> RegisterBank {
        RegisterBank {
            registers: [0; ::std::u8::MAX as usize]
        }
    }
}

impl Index<Register> for RegisterBank {
    type Output = u32;
    fn index(&self, idx: Register) -> &Self::Output {
        &self.registers[idx as usize]
    }
}

impl IndexMut<Register> for RegisterBank {
    fn index_mut(&mut self, idx: Register) -> &mut Self::Output {
        &mut self.registers[idx as usize]
    }
}

/// Memory
/// 
/// 
/// The following is taken from the ARMv7-M Architecture Reference Manual:
/// 
/// This address space is regarded as consisting of 2^30 32-bit words, each of whose addresses is word-aligned, meaning
/// that the address is divisible by 4. The word whose word-aligned address is A consists of the four bytes with
/// addresses A, A+1, A+2, and A+3. The address space can also be considered as consisting of 2^31 16-bit halfwords,
/// each of whose addresses is halfword-aligned, meaning that the address is divisible by 2. The halfword whose
/// halfword-aligned address is A consists of the two bytes with addresses A and A+1.
/// 
/// Address calculations are normally performed using ordinary integer instructions. This means that they normally
/// wrap around if they overflow or underflow the address space. Another way of describing this is that any address
/// calculation is reduced modulo 2^32 .
/// 
/// Normal sequential execution of instructions effectively calculates:
///   (address_of_current_instruction) + (2 or 4)    /*16- and 32-bit instr mix*/
/// after each instruction to determine which instruction to execute next. If this calculation overflows the top of the
/// address space, the result is UNPREDICTABLE . In ARMv7-M this condition cannot occur because the top of memory
/// is defined to always have the Execute Never (XN) memory attribute associated with it. See The system address map
/// on page B3-592 for more details. An access violation will be reported if this scenario occurs.
///
/// All memory addresses used in ARMv7-M are physical addresses (PAs).
///
///
///
/// SYSTEM ADDRESS MAP (B3.1 pg. 592)
///   The architecture assigns physical addresses for use as event entry points (vectors), system control, and
///   configuration. The event entry points are all defined relative to a table base address, that is configured to an
///   IMPLEMENTATION DEFINED value on reset, and then maintained in an address space reserved for system
///   configuration and control. To meet this and other system needs, the address space 0xE0000000 to 0xFFFFFFFF is
///   RESERVED for system-level use.
///
/// | Address                    | Name       | Device type | XN? | Cache | Description                                 |
/// | [0x00000000 -> 0x1FFFFFFF] | Code       | Normal      | -   | WT    | Typically ROM or flash memory.              |
/// | [0x20000000 -> 0x3FFFFFFF] | SRAM       | Normal      | -   | WBWA  | SRAM region typically used for on-chip RAM. |
/// | [0x40000000 -> 0x5FFFFFFF] | Peripheral | Device      | XN  | -     | On-chip peripheral address space.           |
pub struct Memory {
    raw: Box<[u8]>,

}

impl Memory {
    pub fn new() -> Memory {
        let alloc_size = 1024 * 16; // bytes
        let raw: Box<[u8]> = Vec::with_capacity(alloc_size).into_boxed_slice();

        Memory {
            raw: raw
        }
    }

    pub fn map(&mut self, address: usize, bytes: usize, align: usize) {
        
    }
}
