use super::token;
use std::collections::HashMap;

pub struct Template<'a> {
    pub read_position: u32,
    pub position: u32,
    pub input: &'a str,
    pub data: HashMap<&'a str, &'a str>,
    pub ch : Option<char>,
    pub marks : Vec<(Option<u32>, Option<u32>)>,
}

pub trait Lexer {
    fn read(&mut self);
    fn read_char(&mut self);
    fn next_token(&mut self) -> token::Token;
}

pub trait Parser {
    fn mark(&mut self);
}

impl<'a> Template<'a> {
    pub fn new(input: &'a str, data: HashMap<&'a str, &'a str>) -> Template<'a> {
        let mut template = Template { 
            input: input,
            read_position: 0,
            position: 0,
            data : data,
            ch: None,
            marks : vec![(None, None)],
        };
        return template
    }
}

impl<'a> Lexer for Template<'a> {
    fn read(&mut self) {
        while self.read_position as usize <= self.input.len() {
            self.next_token();
        }
    }

    fn next_token(&mut self) -> token::Token {
        self.read_char();
        let tok : token::Token = match self.ch {
            None => token::Token::new(token::EOF, self.ch),
            Some('{') => token::Token::new(token::LEFTBRACKET, self.ch),
            Some('}') => token::Token::new(token::LEFTBRACKET, self.ch),
            _=> token::Token::new(token::TEXT, self.ch)
        };
        return tok;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = None
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position as usize).unwrap());
        }
        if self.ch != None {
            println!("{}", self.ch.unwrap());
        }
        self.position += 1;
        self.read_position += 1;
    }
}
