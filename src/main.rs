use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::process::id;
use std::str::Chars;

static mut EXIT_CODE: i32 = 0;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

impl Token {
    fn new(c: char) -> Self {
        match c {
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '\x20'| '\t'=> Token::Atom(c),
            _ => Token::Op(c),
        }
    }

    fn value(self) -> Option<String> {
        None
    }
    fn name(self) -> Option<String> {
        let name = match self {
            Token::Atom(ch) => {
                "Atom Found".to_string()
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
                    '=' => "EQUAL",
                    '!' => "BANG",
                    '<' => "LESS",
                    '>' => "GREATER",
                    '/' => "SLASH",
                    _ => return None,
                };
                token_name.to_string()
            }
            Token::Eof => "EOF".to_string(),
        };
        Some(name)
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
    unsafe {
        process::exit(EXIT_CODE);
    }
}

fn print_tokens(input: &String) {
    // Uncomment this block to pass the first stage
    // if input.is_empty() {
    //     println!("EOF  null");
    // }
    let mut tokens = input.chars().peekable();
    let mut line_number = 1;
    let mut error = String::new();
    let mut identified = String::new();
    loop {
        {
            match tokens.next() {
                Some(mut t) => {
                    if t == '\n' {
                        line_number = line_number + 1;
                        continue;
                    };
                    let mut operator = String::new();
                    operator.push(t);
                    let token = Token::new(t);
                    let name = token.name();
                    let value = token.value().unwrap_or("null".to_string());
                    // let
                    match name {
                        None => {
                            unsafe {
                                EXIT_CODE = 65;
                            }
                            error.push_str(&format!(
                                "[line {}] Error: Unexpected character: {}\n",
                                line_number, t
                            ));
                        }
                        Some(mut name) => {
                            match (tokens.peek(), t) {
                                (Some('='), current) => {
                                    if matches!(current, '=' | '!' | '>' | '<') {
                                        name = Token::new(current).name().unwrap() + "_EQUAL";
                                        operator.push(tokens.next().unwrap());
                                    }
                                }
                                (Some('/'), '/') => {
                                    // identified.push_str(&format!("EOF  null"));
                                    // break;
                                    
                                }
                                (Some(_), '\t')=>
                                {
                                    continue;
                                }
                                (Some(_), '\x20')=>
                                {
                                    continue;
                                }
                                (_, _) => {}
                            }
                            identified.push_str(&format!("{} {} {}\n", name, operator, value));
                            // identified.push('\n');
                        }
                    };
                }
                None => {
                    identified.push_str(&format!("EOF  null"));
                    break;
                }
            }
        }
    }
    eprint!("{}", error);
    print!("{}", identified);
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
    assert_eq!("LEFT_PAREN", Token::Op('(').name().unwrap())
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

// fn test_print_tokens() {

// }
