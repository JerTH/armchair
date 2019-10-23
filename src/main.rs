use armv7m_vm::instructions::InstrThumb16;
use armv7m_vm::processor::Processor;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::result::Result<(), ()> {
    generate_decode_table().unwrap();
    run_test_program()?;
    Ok(())
}

fn run_test_program() -> std::result::Result<(), ()> {
    let mut processor = Processor::new();
    processor.init();
    processor.load(&[0xBF00, 0xBF00, 0x0001]);
    processor.run()?;
    Ok(())
}

fn generate_decode_table() -> std::io::Result<()> {
    let set = InstrThumb16::generate_decode_table();

    let mut file = File::create("decode_table.rs")?;

    for (idx, item) in set.iter().enumerate() {
        writeln!(file, "{:#06x} // {:?} {:#018b}", idx, item, idx)?;
    }

    Ok(())
}
