#![allow(dead_code)]
use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Def,
    End,
    Identifier(String),
    Integer(i32),
    Oparen,
    Cparen,
    Comma,
}

pub fn tokenize(code: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut code = &code[..];
    while code.len() > 0 {
        if let Some(found) = Regex::new(r"\A\bdef\b").unwrap().find(code) {
            code = code[found.end()..].trim();
            tokens.push(Token::Def);
        } else if let Some(found) = Regex::new(r"\A\bend\b").unwrap().find(code) {
            code = code[found.end()..].trim();
            tokens.push(Token::End);
        } else if let Some(found) = Regex::new(r"\A\b[a-zA-z]+\b").unwrap().find(code) {
            tokens.push(Token::Identifier(
                code[found.start()..found.end()].to_owned(),
            ));
            code = code[found.end()..].trim();
        } else if let Some(found) = Regex::new(r"\A\b[0-9]+\b").unwrap().find(code) {
            tokens.push(Token::Integer(
                code[found.start()..found.end()].parse::<i32>().unwrap(),
            ));
            code = code[found.end()..].trim();
        } else if let Some(found) = Regex::new(r"\A\(").unwrap().find(code) {
            code = code[found.end()..].trim();
            tokens.push(Token::Oparen);
        } else if let Some(found) = Regex::new(r"\A\)").unwrap().find(code) {
            code = code[found.end()..].trim();
            tokens.push(Token::Cparen);
        } else if let Some(found) = Regex::new(r"\A,").unwrap().find(code) {
            code = code[found.end()..].trim();
            tokens.push(Token::Comma);
        }
    }

    tokens
}
