
mod mem;
mod parser;
mod common;
mod semantic;
mod codegen;
mod interpreter;

use parser::parse_file;

fn main() {
    println!("Hello");
    let _ast = parse_file("./testdata/example.lp").unwrap();
}
