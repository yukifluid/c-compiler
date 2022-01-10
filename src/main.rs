mod config;
mod token;
mod lexer;

use std::{env, process};
use lexer::Lexer;

use crate::config::Config;
use crate::token::Token;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut lexer = Lexer::new(&config);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    while let Ok(token) = lexer.lex() {
        match token {
            // 演算子
            Token::OP(val) => {
                match val {
                    "+" => print!("  add rax, "),
                    "-" => print!("  sub rax, "),
                    _ => panic!("invalid op")
                };

                if let Ok(token) = lexer.lex() {
                    if let Token::NUM(val) = token {
                        println!("{}", val);
                    }
                }
            },
            // 数値
            Token::NUM(val) => {
                println!("  mov rax, {}", val);
            }
            // EOF
            Token::EOF => {
                break;
            }
        }
    }

    println!("  ret");
}