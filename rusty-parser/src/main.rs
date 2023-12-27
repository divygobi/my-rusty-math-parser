use std::{io::{self, stdin, Read}, error};

const VALID_CHARS: &str = "0123456789+-*/() ";

fn main() {
    let mut input = String::new();
    println!("Please type some input to be parsed");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            clean_input(&input);
        }
        Err(error) => {
            println!("Error; {}", error);
        }
    }
    println!("Hello, world!");

}

fn clean_input(input: &String) {
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