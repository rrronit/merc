use clap::Parser;
use colored::*;
use merc::repl;
use miette::{IntoDiagnostic, WrapErr};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(short, long)]
    filename: Option<PathBuf>,

    #[clap(short, long)]
    tokenize: Option<bool>,

    #[clap(short, long)]
    parser: Option<bool>,

    #[clap(short, long)]
    interpret: Option<bool>,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match (args.tokenize, args.parser, args.interpret) {
        (Some(true), _, _) => {
            match args.filename {
                Some(filename) => {
                    let contents = fs::read_to_string(&filename)
                        .into_diagnostic()
                        .wrap_err_with(|| format!("Failed to read file: {}", filename.display()))?;

                    let lexer = merc::Lexer::new(&contents);

                    for token in lexer {
                        let token = token?;
                        println!("{}", token);
                    }
                }

                None => {}
            };
            Ok(())
        }
        (_, Some(true), _) => {
            match args.filename {
                Some(filename) => {
                    let contents = fs::read_to_string(&filename)
                        .into_diagnostic()
                        .wrap_err_with(|| format!("Failed to read file: {}", filename.display()))?;

                    let mut parser = merc::Parser::new(&contents);

                    while let Some(ast) = parser.parse_statement() {
                        match ast {
                            Ok(ast) => println!("{:?}", ast),
                            Err(err) => {
                                eprintln!("{:?}", err);
                                break;
                            }
                        }
                    }
                }

                None => {}
            };
            Ok(())
        }
        (_, _, Some(true)) => match args.filename {
            Some(filename) => {
                let contents = fs::read_to_string(&filename)
                    .into_diagnostic()
                    .wrap_err_with(|| format!("Failed to read file: {}", filename.display()))?;

                let parser = merc::Parser::new(&contents);

                let mut a = merc::Interpreter::new(parser);
                let _ = a.run();

                Ok(())
            }
            None => Ok(()),
        },
        (None, None, None) => {
            repl::repl();
            Ok(())
        }
        (_, _, _) => Ok(()),
    }
}
