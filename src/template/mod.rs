use super::token;
use super::data::Data;
use std::collections::HashMap;

pub struct Pattern<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

impl<'a> Pattern<'a> {
    fn new(left: token::TokenType<'a>, right: token::TokenType<'a>) -> Self {
        Self {
            left: left,
            right: right
        }
    }
}

pub trait Lexer {
    fn read(&mut self);
    fn read_char(&mut self);
    fn next_token(&mut self) -> token::Token;
}

pub trait Parser {
    fn parse(&mut self) -> Result<&str, &str>;
    fn mark(&mut self); 
}

pub struct Template<'a> {
    pub read_position: u32,
    pub position: u32,
    pub input: &'a str,
    pub data: Data<'a>,
    pub ch : Option<char>,
    pub marks : Vec<(Option<u32>, Option<u32>)>,
    pub tokens : Vec<token::Token>,
}

impl<'a> Template<'a> {
    pub fn new(input: &'a str, data: Data<'a>, pattern: Pattern) -> Self {
        let mut template = Self { 
            input: input,
            read_position: 0,
            position: 0,
            data : data,
            ch: None,
            marks : vec![],
            tokens : vec![],
        };
        return template
    }

    pub fn compile(&mut self) {
        self.read();
    }
}

impl<'a> Lexer for Template<'a> {

    fn read(&mut self) {
        let mut v : Vec<token::Token> = vec![];
        while self.read_position as usize <= self.input.len() {
            let tok = self.next_token();
            v.push(tok);
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
        
        self.position += 1;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::token;
    use super::Template;
    use super::Data;
    use super::Pattern;

    #[test]
    fn test_template_create() {
        let st: &str = "my new template {test}";
        let mut d = Data::new();
        let mut t = Template::new(st, d, Pattern::new(token::LEFTBRACKET, token::RIGHTBRACKET));
    }

    #[test]
    fn test_read() {
        let st: &str = "my new template {butt}";
        let mut d = Data::new();
        d.add_many(vec![("one","two"), ("two", "three")]);
        let mut t = Template::new(st, d,  Pattern::new(token::LEFTBRACKET, token::RIGHTBRACKET));
        t.compile();
        assert_eq!(t.read_position - 1, st.len() as u32);
        assert_eq!(t.input, st);
        assert_eq!(t.data.data.len() as u32, 2); 
    }
}
