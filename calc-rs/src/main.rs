use std::{
    error::Error,
    io::{self, Write},
};

use clap::Parser as clap_Parser;

use tree_sitter::Parser;
mod calc_std;
mod fmt;
mod interp;
mod nodes;

#[derive(clap_Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// calc program to run
    program: Option<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if let Some(program) = cli.program {
        run(program, false, cli.debug)?;
    } else {
        loop {
            let source = get_input()?;
            // Get the input
            if source == "exit" || source.len() == 0 {
                break;
            }
            run(source, true, cli.debug)?;
        }
    }

    Ok(())
}

fn run(source: String, repl: bool, debug: bool) -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_calc::LANGUAGE.into();
    parser.set_language(&lang)?;

    let mut interpreter = interp::Interp::new();
    let tree = parser.parse(&source, None).unwrap();
    // run
    interpreter.run(source, tree.root_node(), repl, debug)?;
    Ok(())
}

fn get_input() -> io::Result<String> {
    let mut source = String::new();
    print!("> ");
    io::stdout().flush()?;

    io::stdin()
        .read_line(&mut source)
        .expect("Failed to read line");

    let source = source.trim();
    Ok(source.to_string())
}
