use armv7m_vm::loader::ProgramLoader;
use armv7m_vm::processor::Processor;

fn main() {
    //let program_loader = ProgramLoader::from_elf("thumbv7m-test-binary").unwrap();

    let mut p = Processor::new();
    p.init();
}
