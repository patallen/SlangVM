use regex::Regex;

#[derive(Debug, Clone)]
pub enum LabelType {
    Global,
    Local,
}

#[derive(Debug, Clone)]
pub enum Directive {
    Code,
    Data,
    Space,
    Empty
}

impl Directive {
    pub fn from_string(string: &str) -> Self {
        match &*string.to_lowercase() {
            "@code"  => Directive::Code,
            "@data"  => Directive::Data,
            "@space" => Directive::Space,
            _ => Directive::Empty
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
}

impl Token {
    pub fn from_string(string: &str) -> Result<Token, &str> {
        if Regex::new(r"^@([a-zA-Z]+$)").unwrap().is_match(string) {
            return Ok(Token::Directive(Directive::from_string(string)))
        }
        if Regex::new(r"^\.\w+:$").unwrap().is_match(&string[0..string.len()]){
            let ret = &string[0..string.len()-1];
            return Ok(Token::Label(LabelType::Global, ret.to_owned()))
        }
        else if Regex::new(r"^'\w+:$").unwrap().is_match(string){
            let ret = &string[0..string.len()-1];
            return Ok(Token::Label(LabelType::Local, ret.to_owned()))
        }
        else if Regex::new(r"^\.\w+$").unwrap().is_match(string) {
            return Ok(Token::Reference(LabelType::Global, string.to_owned()))
        }
        else if Regex::new(r"^'\w+$").unwrap().is_match(string){
            return Ok(Token::Reference(LabelType::Local, string.to_owned()))
        }
        else if Regex::new(r"^[a-zA-Z]+$").unwrap().is_match(string) {
            return Ok(Token::Instruction(string.to_string()))
        }
        else if Regex::new(r"^-?\d*\.?\d+$").unwrap().is_match(string){
            return Ok(Token::Constant(string.parse().unwrap()))
        }
        else if Regex::new(r"^\n$").unwrap().is_match(string) {
            return Ok(Token::NewLine)
        }
        else if Regex::new(r"^;.+").unwrap().is_match(string) {
            let ret = string.trim_left_matches(';').trim_left();
            return Ok(Token::Comment(ret.to_owned()))
        }
        Err("nope")
    }
}

pub struct Lexer {
    source: String,
    offset: usize,
    base_index: usize,
}
impl Lexer {
    pub fn new(source: String) -> Self {
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
            let token = Token::from_string(string);
            match token {
                Ok(tok) => {
                    tokens.push(tok);
                },
                Err(_) => panic!("Invalid token '{}'", string),
            }
        }
        tokens
    }
}
