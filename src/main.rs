use cemitter::Emitter;
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
    // idk what I meant by the above comment, perhaps im remarking about how amazed I am by
    // rust's string handling. idk
    let lexer = Lexer::new(&match String::from_utf8(input_file) {
        Ok(o) => o,
        Err(e) => panic!(
            "Internal error, failed to convert input file to String: {}",
            e
        ),
    });

    // lets init an emitter as well
    let mut emitter = Emitter::new("out.c".to_string());
    // init parser
    let mut parser = Parser::new(lexer, &mut emitter);

    // allow parser to parse
    parser.program();
    println!("Parsing complete");
    // allow emitter to emit
    emitter.write_file();

    // allow me fam
    println!("Compiling completed.");
}
