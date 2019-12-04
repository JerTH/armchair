use std::path::Path;

extern crate elfy;
use elfy::{ Elf, ParseElfResult };

use crate::memory::Memory;

#[derive(Debug)]
pub struct ProgramLoader {
    elf: Elf,
}

impl ProgramLoader {
    pub fn from_elf<P: AsRef<Path>>(path: P) -> ParseElfResult<ProgramLoader> {
        let elf = Elf::load(path)?;
        let loader = ProgramLoader {
            elf: elf
        };

        Ok(loader)
    }

    pub fn map_to(&self, memory: &Memory) {
        
    }
}

// Executable and shared object files have a base address, which is the lowest virtual address associated with
// the memory image of the program’s object file. One use of the base address is to relocate the memory
// image of the program during dynamic linking.
// 
// An executable or shared object file’s base address is calculated during execution from three values: the
// memory load address, the maximum page size, and the lowest virtual address of a program’s loadable
// segment. As ‘‘Program Loading’’
fn calculate_base_address() {
    unimplemented!()
}

