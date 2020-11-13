use std::fs;

use wat_ast;

fn main() {
    let s = fs::read_to_string("path_open.wat").unwrap();
    let buf = wast::parser::ParseBuffer::new(&s).unwrap();
    let module = wast::parser::parse::<wat_ast::Document>(&buf).unwrap();

    println!("{:#?}", module);
}
