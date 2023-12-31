use std::{io::{self, stdin, Read}, error, fmt::Error};
const VALID_CHARS: &str = "0123456789+-*/() ";

#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Minus,
    Divide,
    Number(u64),
    Paren
}

fn main() {
    let mut input = String::new();
    println!("Please type some input to be parsed");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            check_input(&input);
        }
        Err(error) => {
            println!("Error; {}", error);
        }
    }
    println!("Hello, world!");
}

fn check_input(input: &String) {
    let input = input.trim();
    match input.chars().all(|c| VALID_CHARS.contains(c)){
        true => {
            println!("Your input is {}, and it can be parsed!", input);
        }
        false => {
            println!("Please fix your input");
            println!("Only the following inputs are valid: {}", VALID_CHARS);
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<GrammarItem>, String> {
    let mut tokens: Vec<GrammarItem> = vec![];
    for c in input.chars(){
        match c {
            '*' => tokens.push(GrammarItem::Product),
            '/' => tokens.push(GrammarItem::Divide),
            '-' => tokens.push(GrammarItem::Minus),
            '+' => tokens.push(GrammarItem::Sum),
            ')'|'(' => tokens.push(GrammarItem::Paren),
            //relies on the fact of a clean input
            //TODO raise an error message for a invalid inputs
            '0'..='9' => tokens.push(GrammarItem::Number(c as u64 - '0' as u64)),
            ' ' => {},
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(tokens)
}