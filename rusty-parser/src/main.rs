use core::num;
use std::{io::{self, stdin, Read}, error, fmt::Error, vec, collections::vec_deque};
const VALID_CHARS: &str = "0123456789+-*/() ";

#[derive(Debug, Clone, PartialEq)]
pub enum GrammarItem {
    Multiply,
    Plus,
    Minus,
    Divide,
    Number(u64),
    LeftParen,
    RightParen
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
    let tokens: Vec<GrammarItem> = tokenize(&input).unwrap();
    println!("Hello, world!");
    
}

fn check_input(input: &String) {
    let input = input.trim();
    if !input.chars().all(|c| VALID_CHARS.contains(c)) {
        println!("Please fix your input");
        println!("Only the following inputs are valid: {}", VALID_CHARS);
        return;
    }

    //leetcode actually useful in something lmao
    let mut stack: Vec<char> = vec!();
    for c in input.chars() {
        if c == '('{
            stack.push(')');
        }
        else if c == stack[stack.len()-1] {
            stack.pop();
        }
        if !stack.is_empty(){
            println!("Please fix your input");
            println!("Make sure your parentheses make sense")
        }

    }

    println!("Your input is {}, and it can be parsed!", input);

}



fn tokenize(input: &str) -> Result<Vec<GrammarItem>, String> {
    let mut tokens: Vec<GrammarItem> = vec![];
    let mut num_buffer = String::new();

    for c in input.chars() {
        if !c.is_digit(10) {
            flush_number_buffer(&mut num_buffer, &mut tokens)?;
        }

        match c {
            '*' => tokens.push(GrammarItem::Multiply),
            '/' => tokens.push(GrammarItem::Divide),
            '-' => tokens.push(GrammarItem::Minus),
            '+' => tokens.push(GrammarItem::Plus),
            '(' => tokens.push(GrammarItem::LeftParen),
            ')' => tokens.push(GrammarItem::RightParen),
            '0'..='9' => num_buffer.push(c),
            ' ' => {}, // space is already handled by the flush check
            _ => return Err(format!("unexpected character {}", c)),
        }
    }

    // Flush any remaining number at the end of the input
    flush_number_buffer(&mut num_buffer, &mut tokens)?;

    Ok(tokens)
}

fn flush_number_buffer(num_buffer: &mut String, tokens: &mut Vec<GrammarItem>) -> Result<(), String> {
    if !num_buffer.is_empty() {
        match num_buffer.parse::<u64>() {
            Ok(num) => tokens.push(GrammarItem::Number(num)),
            Err(_) => return Err(format!("invalid number: {}", num_buffer)),
        }
        num_buffer.clear();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_single_digit_add() {
        const basic_add: &str = "2+2";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(2),
            GrammarItem::Plus,
            GrammarItem::Number(2),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(basic_add).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn basic_single_digit_subtract() {
        const basic_subtract: &str = "2-3";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(2),
            GrammarItem::Minus,
            GrammarItem::Number(3),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(basic_subtract).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn basic_single_digit_multiply() {
        const basic_multiply: &str = "4*5";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(4),
            GrammarItem::Multiply,
            GrammarItem::Number(5),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(basic_multiply).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn basic_single_digit_divide() {
        const basic_divide: &str = "6/3";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(6),
            GrammarItem::Divide,
            GrammarItem::Number(3),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(basic_divide).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn multiple_digits_add() {
        const add: &str = "12+34";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(12),
            GrammarItem::Plus,
            GrammarItem::Number(34),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(add).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn multiple_digits_subtract() {
        const subtract: &str = "56-78";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(56),
            GrammarItem::Minus,
            GrammarItem::Number(78),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(subtract).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn multiple_digits_multiply() {
        const multiply: &str = "90*12";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(90),
            GrammarItem::Multiply,
            GrammarItem::Number(12),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(multiply).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }

    #[test]
    fn multiple_digits_divide() {
        const divide: &str = "34/56";
        let tokens_correct: Vec<GrammarItem> = vec![
            GrammarItem::Number(34),
            GrammarItem::Divide,
            GrammarItem::Number(56),
        ];
        let tokens_test: Vec<GrammarItem> = tokenize(divide).unwrap();
        assert_eq!(tokens_correct, tokens_test);
    }
}
