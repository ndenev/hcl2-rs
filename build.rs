use std::env;
use std::path::PathBuf;

fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut grammar_dir = PathBuf::from(src_dir);
    grammar_dir.push("grammar");
    lrpeg::process_files(&grammar_dir, &PathBuf::from(out_dir));
}
