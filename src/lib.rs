#![allow(dead_code)]
use regex::Regex;

#[derive(Debug)]
pub enum TokenType {
    Def,
    End,
    Identifier(String),
    Integer(i32),
    Oparen,
    Cparen,
    Comma,
}

pub fn tokenize(code: String) -> Vec<TokenType> {
    let tokens_regex = [
        (TokenType::Def, r"\bdef\b"),
        (TokenType::End, r"\bend\b"),
        (TokenType::Identifier(String::new()), r"\b[a-zA-Z]+\b"),
        (TokenType::Integer(0), r"\b[0-9]+\b"),
        (TokenType::Oparen, r"\("),
        (TokenType::Cparen, r"\)"),
        (TokenType::Comma, r","),
    ];

    let mut tokens = vec![];
    let mut code = &code[..];
    while code.len() > 0 {
        tokens_regex.iter().for_each(|(token_type, re)| {
            let re = &(r"\A".to_owned() + re)[..];
            if let Some(f) = Regex::new(re).unwrap().find(code) {
                let value = &code[f.start()..f.end()];
                tokens.push(match token_type {
                    TokenType::Def => TokenType::Def,
                    TokenType::End => TokenType::End,
                    TokenType::Identifier(_) => TokenType::Identifier(String::from(value)),
                    TokenType::Integer(_) => TokenType::Integer(value.parse::<i32>().unwrap()),
                    TokenType::Oparen => TokenType::Oparen,
                    TokenType::Cparen => TokenType::Cparen,
                    TokenType::Comma => TokenType::Comma,
                });
                code = code[f.end()..].trim();
            }
        });
    }

    tokens
}
