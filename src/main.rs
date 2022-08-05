mod mem;
mod parser;
mod common;
mod semantic;
mod codegen;
mod interpreter;

fn main() {
    parser::parse_file("./testdata/example.lp");
}
