use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("./write_trait.txt").unwrap();

    let writer: &mut dyn Write = &mut file;
    writer.write_all(b"hello").unwrap();

}