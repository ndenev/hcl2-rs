include!(concat!(env!("OUT_DIR"), "/hcl2.rs"));

use std::{error::Error, fs::File, io::Read};

use hcl2::{Node, Rule};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = hcl2::PEG::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage {} EXPRESSION", &args[0]);
        std::process::exit(2);
    }

    let input = &args[1];

    println!("parsing: {}", input);

    let mut data = String::new();

    let mut input_file = File::open(input)?;

    let size = input_file.read_to_string(&mut data)?;

    println!("read {size} bytes from {input}");

    match parser.parse(&data) {
        Ok(node) => {
            println!("Node: {node:?}")
        }
        Err((line_no, col_no)) => {
            eprintln!("parser error at {}:{}", line_no, col_no);
        }
    }

    Ok(())
}
