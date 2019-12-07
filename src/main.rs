use armchair::loader::ProgramLoader;
use armchair::processor::Processor;

fn main() {
    let image = ProgramLoader::load("thumbv7m-test-binary").unwrap();
    let mut processor = Processor::new();
    processor.init();
    processor.load(image);
    processor.reset();
    processor.run();
}
