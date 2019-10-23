use armv7m_vm::decode::test_instruction_macro;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let set = test_instruction_macro();

    let mut file = File::create("decode_table.rs")?;

    for item in &set {
        writeln!(file, "{:#04x} // {:?} {:#018b}", item.0, item, item.0)?;
    }

    Ok(())
}
