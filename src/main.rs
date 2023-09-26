#![allow(unused)]

use std::{env, path::Path, fs::File, io::{Read, stdin, stdout, Write}, char};
use std::iter::Peekable;
mod lexer;

#[derive(Clone)]
enum Op {
    Plus,
    Minus,
    Slash,
    Star,
    Greater,
    Less,
    EqualEqual,
    NotEqual,
    GreaterEq,
    LessEq,
}
impl Op {
    fn from_token_kind(tk: lexer::TokenKind) -> Self {
        use lexer::TokenKind::*;
        match tk {
            Minus => Op::Minus,
            Plus => Op::Plus,
            Slash => Op::Slash,
            Star => Op::Star,
            Greater => Op::Greater,
            Less => Op::Less,
            EqualEqual => Op::EqualEqual,
            NotEqual => Op::NotEqual,
            GreaterEq => Op::GreaterEq,
            LessEq => Op::LessEq,
            _ => panic!("wrong token kind"),
        }
    }
}

#[derive(Clone)]
enum UnaryKind {
    Bang,
    Minus,
}
impl UnaryKind {
    fn from_token_kind(tk: lexer::TokenKind) -> Self {
        match tk {
            lexer::TokenKind::Bang => UnaryKind::Bang,
            lexer::TokenKind::Minus => UnaryKind::Minus,
            _ => panic!("wrong token kind."),
        }
    }
}

// This is Expr part.
#[derive(Clone)]
enum Expr {
    Literal(lexer::Token),
    Variable(String),
    Function(String, Vec<Expr>),
    Binary(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryKind, Box<Expr>),
}
impl Expr {
    fn parse(tokens: &mut impl Iterator<Item=lexer::Token>) {
        let mut ast: Vec<Expr> = Vec::new();
        let mut tokens_iter = tokens;

        use lexer::TokenKind::*;
        while let Some(t) = tokens_iter.next() {
            match t.kind {
                Puts => {
                    let tree = Expr::parse_expression(&mut tokens_iter);
                },

                
                _ => todo!(),

            }
        }
    }

    fn parse_factor(val: &mut impl Iterator<Item=lexer::Token>) -> Option<Expr> {
        let current = val.next().unwrap();
        use lexer::TokenKind::*;
        match current.kind {
            Identifier => Some(Expr::Literal(current)),
            Number => Some(Expr::Literal(current)),
            OpenParen => {
                let a = Expr::parse_expression(&mut val.peekable());
                match val.next().unwrap().kind {
                    CloseParen => Some(a),
                    _ => None,
                }
            },
            Minus => Self::parse_factor(val),
            _ => None,
        }
    }
    fn parse_expression(val: &mut impl Iterator<Item=lexer::Token>) -> Expr {
        let mut a = Expr::parse_term(val);
        use lexer::TokenKind::*;
        loop {
            let current = val.next().unwrap();
            let res = match current.kind {
                Plus => {
                    let b = Expr::parse_term(val);
                    a = Expr::Binary(Op::Plus, Box::new(a), Box::new(b));
                    Some(())
                },
                Minus => {
                    let b = Expr::parse_term(val);
                    a = Expr::Binary(Op::Minus, Box::new(a), Box::new(b));
                    Some(())
                },
                _ => None,
            };
            if let None = res {
                return a;
            }
        }
    }
    fn parse_term(val: &mut impl Iterator<Item = lexer::Token>) -> Expr {
        let mut a = Expr::parse_factor(val).unwrap();
        use lexer::TokenKind::*;
        loop {
            let current = val.next().unwrap();
            let res = match current.kind {
                Star => {
                    let b = Expr::parse_factor(val).unwrap();
                    a = Expr::Binary(Op::Plus, Box::new(a), Box::new(b));
                    Some(())
                },
                Slash => {
                    let b = Expr::parse_factor(val).unwrap();
                    a = Expr::Binary(Op::Minus, Box::new(a), Box::new(b));
                    Some(())
                },
                _ => None,
            };
            if let None = res {
                return a;
            }
        }
    }
}




fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox <script>");
    } else if args.len() == 2 {
        let path = Path::new(&args[1]);
        if path.is_file() {
            run_file(&path);
        } else {
            println!("Error: Input file not found.");
        }
    } else {
        run_prompt();
    }
}

fn run_file(path: &Path) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    run(contents);
}
fn run_prompt() {
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("cannot read line");

        if input.is_empty() {
            break;
        }

        run(input.clone());
    }
}

fn run(contents: String) {
    let tokens = lexer::full_trim_tokens(lexer::scanner(contents));
}

fn error(message: String, line: u32) {
    panic!("Error: {} at line {}", message, line);
}
