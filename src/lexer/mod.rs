use super::data;
use super::token;

pub trait Lexing {
    fn read(&mut self);
    fn read_char(&mut self);
    fn next_token(&mut self) -> token::Token;
    fn peek_char(&mut self)-> Option<char>;
}

pub struct Lexer<'a>{
    pub read_position: u32,
    pub input: &'a str,
    pub tokens : Vec<token::Token>,
    pub pattern: token::Pattern,
    pub position: u32,
    pub data: data::Data,
    pub ch : Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, data: data::Data, pattern: token::Pattern) -> Self {
        Self { 
            input: input,
            read_position: 0,
            pattern: pattern,
            position: 0,
            data : data,
            ch: None,
            tokens : vec![],
        }
    }
}

impl<'a> Lexing for Lexer<'a> {
    fn read(&mut self) {
        let mut v : Vec<token::Token> = vec![];
        while self.read_position as usize <= self.input.len() {
            let tok = self.next_token();
            v.push(tok);
        }
        self.tokens = v;
    }

    fn next_token(&mut self) -> token::Token {
        self.read_char();
        let tok : token::Token = match self.ch {
            None => token::Token::new(token::EOF, self.ch),
            Some('{') => {
                    if self.peek_char() != None && self.peek_char().unwrap() == '{' && self.pattern.left == token::DOUBLELEFTBRACKET {
                        let mut t : token::Token = token::Token::new(token::DOUBLELEFTBRACKET, None);
                        t.literal = self.ch.unwrap().to_string() + &self.peek_char().unwrap().to_string();
                        self.read_char();
                        return t;
                    }
                    if self.pattern.left == token::LEFTBRACKET {
                        return token::Token::new(token::LEFTBRACKET, self.ch)
                    }
                    return token::Token::new(token::TEXT, self.ch)
                },
            Some('}') => {
                    if self.peek_char() != None && self.peek_char().unwrap() == '}' && self.pattern.right == token::DOUBLERIGHTBRACKET {
                        let mut t : token::Token = token::Token::new(token::DOUBLERIGHTBRACKET, None);
                        t.literal = self.ch.unwrap().to_string() + &self.peek_char().unwrap().to_string();
                        self.read_char();
                        return t;
                    }
                    if self.pattern.right == token::RIGHTBRACKET {
                        return token::Token::new(token::RIGHTBRACKET, self.ch)
                    }
                    return token::Token::new(token::TEXT, self.ch)
                },
            Some(' ') => token::Token::new(token::SPACE, self.ch),
            _=> token::Token::new(token::TEXT, self.ch),
        };
        return tok;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = None
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position as usize).unwrap());
        }
        
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() as u32 {
            return None
        } else {
            return Some(self.input.chars().nth(self.read_position as usize).unwrap());
        }
    }
}
