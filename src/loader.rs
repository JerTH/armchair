use std::fs::File;

struct ProgramLoader {
    magic_number: usize,

}

impl ProgramLoader {

    fn load(file: &str) {
        let file = File::open(file);
        
    }
}