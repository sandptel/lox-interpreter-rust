use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            tokenize(&file_contents);
        }
        "repl" => {
            eprint!("lox repl --> ");
            repl();
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

// enum Token{
//     Eof,
//     left_paren,
//     light_paren,
// }

fn tokenize(input: &String) {
    // Uncomment this block to pass the first stage
    // if input.is_empty() {
    //     println!("EOF  null");
    // }
    let mut tokens = input.chars();
    loop {
        {
            match tokens.next() {
                Some(t) => {
                    let token = identify_token(&t);
                    println!("{} {} {}",token.0,t,token.1.unwrap_or("null"));
                }
                None => {
                    println!("EOF  null");
                    break;
                }
            }
        }
    }
}

fn identify_token(ch: &char) -> (String, Option<&str>) {
    let mut token_name = match ch {
        '(' => "LEFT_PAREN",
        ')' => "RIGHT_PAREN",
        '{' => "LEFT_BRACE",
        '}' => "RIGHT_BRACE",
        t => "Unknown Command: {t}",
    };
    (token_name.to_string(), None)
}

fn scanner(input: &String) {}

fn repl() {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        tokenize(&input);
        println!("{}", input);
    }
}
