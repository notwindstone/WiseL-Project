mod codegen;
mod input;
mod lexer;
mod parser;

// "use kawaii::*;"
use codegen::*;
use input::*;
use lexer::*;
use parser::*;

fn main() {
    let path: String = get_entry_path();
    let source: String = read_file(&path);
    print_step(Step::READ, source.len());

    tokenize(&source);
    print_step(Step::TOKENIZED, 0);

    parse();
    print_step(Step::PARSED, 0);

    generate();
    print_step(Step::GENERATED, 0);
}
