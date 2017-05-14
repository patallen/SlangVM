use std::iter::Peekable;
use std::str::Chars;
use itertools::Itertools;


#[derive(Debug)]
pub enum Token {
    Ident(String),
    Number(f64),
    Operator(String, u8),
    LeftParen,
    RightParen,
}

enum ConsumeType { Ident, Number }

pub struct Tokenizer<'a> {
    string: &'a str,
}
impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            string: input,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        println!("Tokenizing Expression: {}", self.string);
        let mut chars = &mut self.string.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            match chars.peek() {
                Some(&ch) => match ch {
                    'a'...'z' | '_' => tokens.push(consume_ident(&mut chars)),
                    '0'...'9'       => tokens.push(consume_number(&mut chars)),
                    '+' | '-'       => tokens.push(Token::Operator(chars.take(1).collect::<String>(), 2)),
                    '*' | '/'       => tokens.push(Token::Operator(chars.take(1).collect::<String>(), 3)),
                    ')'             => {tokens.push(Token::RightParen); chars.take(1).collect::<String>();},
                    '('             => {tokens.push(Token::LeftParen); chars.take(1).collect::<String>();},
                    ' '             => { chars.take(1).collect::<String>(); }
                    _ => panic!("Syntax Error: {} is not a valid character.", ch)
                },
                None => return tokens
            }
        }
    }
}
fn consume_ident(peekable: &mut Peekable<Chars>) -> Token {
    let ret = peekable.by_ref().peeking_take_while(|x| valid_token_char(x, ConsumeType::Ident)).collect::<String>();
    Token::Ident(ret)
}

fn consume_number(peekable: &mut Peekable<Chars>) -> Token {
    let ret = peekable.by_ref().peeking_take_while(|x| valid_token_char(x, ConsumeType::Number)).collect::<String>();
    Token::Number(ret.parse::<f64>().unwrap())
}

fn valid_token_char(ch: &char, tt: ConsumeType) -> bool {
    match tt {
        ConsumeType::Ident => match *ch {
            'a'...'z' | '_' => true,
            _ => false,
        },
        ConsumeType::Number => match *ch {
            '0'...'9' | '.' => true,
            _ => false,
        },
    }
}

pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Ident(_) | Token::Number(_) => output.push(token),
            Token::Operator(_, cp) => {
                match operators.last() {
                    Some(&Token::Operator(_, lp)) => {
                        if cp > lp { output.push(operators.pop().unwrap()); }
                    }, _ => {}
                };
                operators.push(token);
            },
            Token::LeftParen => { operators.push(token); },
            Token::RightParen => {
                while let Some(operator) = operators.pop() {
                    match operator {
                        Token::LeftParen => { break; },
                        _ => output.push(operator)
                    }
                }
            },
        };
    }
    operators.reverse();
    output.append(&mut operators);
    output
}
