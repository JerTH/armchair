use armv7m_vm::loader::ProgramLoader;
use armv7m_vm::processor::Processor;

fn main() {
    let loader = ProgramLoader::load("thumbv7m-test-binary").unwrap();
}
