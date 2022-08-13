
mod mem;
mod parser;
mod common;
mod semantic;
mod codegen;
mod interpreter;

use parser::parse_file;

fn main() {
    parse_file("./testdata/example.lp");
}
