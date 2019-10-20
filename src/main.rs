
use std::fs::File;
use std::io::prelude::*;

use armv7m_vm::decode::generate_test;

fn main() -> std::io::Result<()>{
    let mut set = generate_test();

    let mut file = File::create("decode_table.rs")?;

    for item in &set {
        writeln!(file, "{:#04x} // {:?} {:#018b}", item.0, item, item.0)?;
    }

    Ok(())
}
