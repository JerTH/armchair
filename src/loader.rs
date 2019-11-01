use std::fs::File;
use std::path::Path;
use std::io::BufReader;

extern crate elfy;
use elfy::Elf;

use crate::memory::Memory;

#[derive(Debug)]
pub struct ProgramLoader;

impl ProgramLoader {
    pub fn load_binary<P: AsRef<Path>>(path: P, _memory: &mut Memory) {
        let file = File::open(path).unwrap();
        let mut buf = BufReader::new(file);
        let elf = Elf::parse(&mut buf).unwrap();
        let code = elf.try_get_section(".text");

        unimplemented!()
    }
}
