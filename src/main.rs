use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(PartialEq, Debug,Clone, Copy)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

impl Token {
    fn new(c: char) -> Self {
        match c {
            '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
            _ => Token::Op(c),
        }
    }

    fn value(self) -> Option<String> {
        None
    }
    fn name(self) -> String {
        let name = match self {
            Token::Atom(ch) => {
                todo!()
            }
            Token::Op(ch) => {
                let mut token_name = match ch {
                    '(' => "LEFT_PAREN",
                    ')' => "RIGHT_PAREN",
                    '{' => "LEFT_BRACE",
                    '}' => "RIGHT_BRACE",
                    '*' => "STAR",
                    '.' => "DOT",
                    ',' => "COMMA",
                    '+' => "PLUS",
                    '-' => "MINUS",
                    ';' => "SEMICOLON",
                    t => &format!("Unknown Command {}", t),
                };
                token_name.to_string()
            }
            Token::Eof => "EOF".to_string(),
        };
        name
    }
}
struct Lexer {
    tokens: Vec<Token>,
}

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
            print_tokens(&file_contents);
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

fn print_tokens(input: &String) {
    // Uncomment this block to pass the first stage
    // if input.is_empty() {
    //     println!("EOF  null");
    // }
    let mut tokens = input.chars();
    loop {
        {
            match tokens.next() {
                Some(t) => {
                    let token = Token::new(t);
                    println!(
                        "{} {} {}",
                        token.name(),
                        t,
                        token.value().unwrap_or("null".to_string())
                    );
                }
                None => {
                    println!("EOF  null");
                    break;
                }
            }
        }
    }
}

fn scanner(input: &String) {}

fn repl() {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        print_tokens(&input);
        println!("{}", input);
    }
}

#[test]

fn test_name() {
    assert_eq!("LEFT_PAREN", Token::Op('(').name())
}

#[test]

fn test_value() {
    assert_eq!("null", Token::Op('(').value().unwrap_or("null".to_string()))
}

#[test]

fn test_new() {
    assert_eq!(Token::new('A'), Token::Atom('A'));
    assert_eq!(Token::new('+'), Token::Op('+'));
}
