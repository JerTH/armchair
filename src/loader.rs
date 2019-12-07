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

    pub fn load<P: AsRef<Path>>(path: P) -> ParseElfResult<ProgramImage> {
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

        let entry = elf.header().entry(); 
        let image = ProgramImage::new(entry, mem);

        println!("[Loader] Program image entry, size: {:#X}, {}", image.entry, image.memory.allocated_bytes());

        Ok(image)
    }

    fn map_segment(mem: &mut Memory, seg: &Segment) {
        let address = seg.header().virtual_address();
        assert_eq!(address, seg.header().physical_address());

        let bytes = seg.data().as_slice();
        assert_eq!(bytes.len(), seg.header().memory_size());

        mem.write_bytes(address, bytes); // todo: error handling using "?"
    }
}

pub struct ProgramImage {
    entry: usize,
    memory: Memory,
}

impl ProgramImage {
    fn new(entry: usize, memory: Memory) -> ProgramImage {
        ProgramImage {
            entry, memory
        }
    }

    pub fn entry(&self) -> usize {
        self.entry
    }

    pub fn into_raw_image(self) -> Memory {
        self.memory
    }
}
