use clap::Parser;
use colored::*;
use merc::Lexer;
use miette::{IntoDiagnostic, WrapErr};
use std::io::Write;
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

                    let lexer = Lexer::new(&contents);

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

                let mut parser = merc::Parser::new(&contents);

                let a = merc::Interpreter::new(parser);

                Ok(())
            }
            None => Ok(()),
        },
        (None, None, None) => {
            print_logo();
            loop {
                print!(">>> ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "exit" || input.trim() == "quit" {
                    println!("{}", "bye bye".bright_yellow().bold());
                    break;
                } else if input.trim() == "help" {
                    println!("{}", "Available commands:".bright_green());
                    println!("  help  - Show this help message");
                    println!("  clear - Clear the screen");
                    println!("  exit  - Exit the interpreter");
                    continue;
                } else if input.trim() == "clear" {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    std::io::stdout().flush().unwrap();
                    print_logo();
                    continue;
                }

                let lexer = Lexer::new(&input);
                for token in lexer {
                    let token = token?;
                    println!("{}", token);
                }
            }
            Ok(())
        }
        (_, _, _) => Ok(()),
    }
}

fn print_logo() {
    println!(
        "{}",
        r#"
 ███▄ ▄███▓▓█████  ██▀███   ▄████▄  
▓██▒▀█▀ ██▒▓█   ▀ ▓██ ▒ ██▒▒██▀ ▀█  
▓██    ▓██░▒███   ▓██ ░▄█ ▒▒▓█    ▄ 
▒██    ▒██ ▒▓█  ▄ ▒██▀▀█▄  ▒▓▓▄ ▄██▒
▒██▒   ░██▒░▒████▒░██▓ ▒██▒▒ ▓███▀ ░
░ ▒░   ░  ░░░ ▒░ ░░ ▒▓ ░▒▓░░ ░▒ ▒  ░
░  ░      ░ ░ ░  ░  ░▒ ░ ▒░  ░  ▒   
░      ░      ░     ░░   ░ ░        
       ░      ░  ░   ░     ░ ░      
                           ░        
"#
        .bright_cyan()
    );

    println!(
        "{}",
        "Welcome to the Merc Interpreter".bright_yellow().bold()
    );
    println!(
        "{}",
        "Type 'exit' or 'quit' to leave the interpreter".italic()
    );
    println!();
}
