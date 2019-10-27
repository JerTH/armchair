use armv7m_vm::processor::Processor;
use armv7m_vm::loader::ProgramLoader;

fn main() {
    let _loader = ProgramLoader::load("thumbv7m-test-binary").unwrap();

    run_test_program();
}

fn run_test_program() {
    //let mut processor = Processor::new();
    //processor.init();
    //processor.load(&[
    //    0xBF00,
    //    0xBF00,
    //    0x0001
    //]);
    //processor.run()
}
