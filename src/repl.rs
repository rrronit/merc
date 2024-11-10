use colored::*;
use std::{collections::HashMap, io::Write};

use crate::{interpreter::Value, Interpreter, Parser};

pub fn repl() {
    print_logo();

    // Create a persistent HashMap to store variables
    let mut variables = HashMap::new();

    loop {
        print!("{} ", ">>".bright_blue());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" | "quit" => break,
            "clear" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                print_logo();
                continue;
            }
            "help" => {
                print_help();
                continue;
            }
            "env" => {
                print_environment(&variables);
                continue;
            }
            _ => {
                let parser = Parser::new(input);
                let mut interpreter = Interpreter::new(parser);

                interpreter.replace_db(variables.clone());

                if let Err(e) = interpreter.run() {
                    println!("{} {}", "Error:".bright_red(), e);
                }

                variables = interpreter.variables.clone();
            }
        }
    }

    println!("{}", "Goodbye!".bright_green());
}



fn print_environment(variables: &HashMap<String, Value>) {
    if variables.is_empty() {
        println!("{}", "No variables defined".bright_yellow());
        return;
    }

    println!("{}", "Current environment:".bright_green());
    for (name, value) in variables {
        println!(
            "  {} = {}",
            name.bright_blue(),
            format!("{}", value).bright_yellow()
        );
    }
}

fn print_help() {
    println!("{}", "Available commands:".bright_green());
    println!("  help  - Show this help message");
    println!("  clear - Clear the screen");
    println!("  exit  - Exit the interpreter");
    println!("  env   - Show all defined variables");
    println!("\nLanguage features:");
    println!("  let x = <expression>  - Define a variable");
    println!("  func name(args) {{ }}  - Define a function");
    println!("  if <cond> {{ }} else {{ }}  - Conditional");
    println!("  while <cond> {{ }}  - Loop");
    println!("  1 + 2 * 3  - Arithmetic");
    println!("  \"hello\" + \" world\"  - String concatenation");
    println!("  true && false  - Boolean operations");
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
}
