use std::{
    env,
    error::Error,
    io::{self, Write},
};
use tree_sitter::Parser;
mod fmt;
mod interp;
mod nodes;
mod rlang_std;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let debug = if args.len() > 1 {
        let d = args[1] == "-d";
        if d {
            println!("debug mode");
        }
        d
    } else {
        false
    };

    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_rlang::LANGUAGE.into();

    parser.set_language(&lang)?;

    let mut source = String::new();
    // let mut i = 0;

    let mut interpreter = interp::Interp::new();
    loop {
        // if i > 1 {
        //     break;
        // }
        source.clear();

        // Get the input
        // let source = if i == 0 { "let x = 3" } else { "x" };
        // i += 1;
        print!("> ");
        io::stdout().flush()?;

        io::stdin()
            .read_line(&mut source)
            .expect("Failed to read line");

        let source = source.trim();
        if source == "exit" || source.len() == 0 {
            break;
        }

        let tree = parser.parse(source, None).unwrap();
        interpreter.set_source(source.to_string());

        // run
        interpreter.run(tree.root_node(), debug)?;
    }

    Ok(())
}
