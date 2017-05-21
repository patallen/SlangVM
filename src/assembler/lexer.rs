use regex::Regex;

lazy_static! {
    static ref REGEX_DIRECTIVE: Regex = Regex::new(r"^@([a-zA-Z]+$)").unwrap();
    static ref REGEX_GLABEL: Regex = Regex::new(r"^\.\w+:$").unwrap();
    static ref REGEX_LLABEL: Regex = Regex::new(r"^'\w+:$").unwrap();
    static ref REGEX_GLABELREF: Regex = Regex::new(r"^\.\w+$").unwrap();
    static ref REGEX_LLABELREF: Regex = Regex::new(r"^'\w+$").unwrap();
    static ref REGEX_INSTRUCTION: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
    static ref REGEX_CONSTANT: Regex = Regex::new(r"^-?\d*\.?\d+$").unwrap();
    static ref REGEX_NEWLINE: Regex = Regex::new(r"^\n$").unwrap();
    static ref REGEX_COMMENT: Regex = Regex::new(r"^;.+").unwrap();
}

#[derive(Debug, Clone)]
pub enum LabelType {
    Global,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Directive {
    Code,
    Data,
    Space,
}

impl Directive {
    pub fn from_string(string: &str) -> Self {
        match &*string.to_lowercase() {
            "@code"  => Directive::Code,
            "@data"  => Directive::Data,
            "@space" => Directive::Space,
            _ => panic!("{} is not not a valid directive.", string)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Label(LabelType, String),
    Reference(LabelType, String),
    Directive(Directive),
    Instruction(String),
    Constant(i64),
    NewLine,
    Comment(String),
    Eof,
}

impl Token {
    pub fn from_string(string: &str) -> Result<Token, String> {
        if REGEX_DIRECTIVE.is_match(string) {
            return Ok(Token::Directive(Directive::from_string(string)))
        } else if REGEX_GLABEL.is_match(&string[0..string.len()]){
            let ret = &string[0..string.len()-1];
            return Ok(Token::Label(LabelType::Global, ret.to_owned()))
        } else if REGEX_LLABEL.is_match(string){
            let ret = &string[0..string.len()-1];
            return Ok(Token::Label(LabelType::Local, ret.to_owned()))
        } else if REGEX_GLABELREF.is_match(string) {
            return Ok(Token::Reference(LabelType::Global, string.to_owned()))
        } else if REGEX_LLABELREF.is_match(string){
            return Ok(Token::Reference(LabelType::Local, string.to_owned()))
        } else if REGEX_INSTRUCTION.is_match(string) {
            return Ok(Token::Instruction(string.to_string()))
        } else if REGEX_CONSTANT.is_match(string){
            return Ok(Token::Constant(string.parse().unwrap()))
        } else if REGEX_NEWLINE.is_match(string) {
            return Ok(Token::NewLine)
        } else if REGEX_COMMENT.is_match(string) {
            let ret = string.trim_left_matches(';').trim_left();
            return Ok(Token::Comment(ret.to_owned()))
        }
        Err(format!("{} did not match any tokens during lexing.", string.clone()))
    }
}

pub struct Lexer<'a> {
    source: &'a String,
    offset: usize,
    base_index: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Lexer {
            source: source,
            offset: 0,
            base_index: 0,
        }
    }
    pub fn next_valid(&mut self) -> &str {
        self.offset = 1;
        let mut current = &self.source[self.base_index..self.base_index+1];
        while current == " " {
            self.base_index += 1;
            current = &self.source[self.base_index..self.base_index+1];
        }
        if current == "\n" { self.base_index += 1; return current; }

        let mut temp = &self.source[self.base_index + self.offset..self.base_index + self.offset + 1];
        if current == ";" {
            while temp != "\n" {
                self.offset += 1;
                temp = &self.source[self.base_index + self.offset..self.base_index + self.offset + 1];
            }
        }
        else {
            while temp != " " && temp != "\n" && temp != ";" {
                self.offset += 1;
                temp = &self.source[self.base_index + self.offset..self.base_index + self.offset + 1];
            }
        }
        let rv = &self.source[self.base_index..self.base_index + self.offset];
        self.base_index += self.offset;
        self.offset = 1;
        return rv
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.base_index < self.source.len() {
            let string = self.next_valid();
            match Token::from_string(string) {
                Ok(tok) => { tokens.push(tok); },
                Err(_) => panic!("Invalid token '{}'", string),
            }
        }
        tokens.push(Token::Eof);
        tokens
    }
}
