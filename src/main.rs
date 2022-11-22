use lexer::Lexer;
use parser::Parser;
use std::env;

fn main() {
    println!("Teeny Tiny Transpiler");

    // get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: Compiler needs a source file as an argument");
        return;
    }

    let input_file = match std::fs::read(args[1].clone()) {
        Ok(o) => o,
        Err(e) => panic!("Failed to read file {}", e),
    };

    // seemingly cursed?
    let lexer = Lexer::new(&match String::from_utf8(input_file) {
        Ok(o) => o,
        Err(e) => panic!(
            "Internal error, failed to convert input file to String: {}",
            e
        ),
    });

    let mut parser = Parser::new(lexer);
    parser.program();

    println!("Parsing complete");
}
