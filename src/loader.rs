use std::path::Path;

extern crate elfy;
use elfy::{ Elf, ParseElfResult };
use elfy::types::{ Segment, ProgramHeaderType, ProgramHeaderFlags };

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

    pub fn load<P: AsRef<Path>>(path: P) -> ParseElfResult<Memory> {
        let elf = Elf::load(path)?;
        
        let mut required_memory = 0;
        for segment in elf.segments() {
            let addr = segment.header().virtual_address();
            let size = segment.header().memory_size();
            required_memory = required_memory.max(addr + size);
        }
        
        let mut mem = Memory::alloc(required_memory);
        for segment in elf.segments() {
            ProgramLoader::map_segment(&mut mem, &segment);
        }

        Ok(mem)
    }

    fn map_segment(mem: &mut Memory, seg: &Segment) {
        let address = seg.header().virtual_address();
        assert_eq!(address, seg.header().physical_address());

        let bytes = seg.data().as_slice();
        assert_eq!(bytes.len(), seg.header().memory_size());

        mem.write_bytes(address, bytes); // todo: error handling using "?"
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

