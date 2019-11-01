use armv7m_vm::memory::Memory;
use armv7m_vm::loader::ProgramLoader;

fn main() {
    let mut memory = Memory::new();
    ProgramLoader::load_binary("thumbv7m-test-binary", &mut memory);
}
