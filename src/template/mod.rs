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

pub struct Mark {
    pub left_token : Option<token::Token>,
    pub right_token: Option<token::Token>,
    pub left_index : Option<usize>,
    pub right_index: Option<usize>,
}


impl Mark {
    fn new() -> Self {
        Self {
            left_token: None,
            right_token: None,
            left_index : None,
            right_index : None,
        }
    }
}

pub trait Lexer {
    fn read(&mut self);
    fn read_char(&mut self);
    fn next_token(&mut self) -> token::Token;
    fn peek_char(&mut self)-> Option<char>;
}

pub trait Parser {
    fn parse(&mut self) -> Result<&str, &str>;
    fn mark(&mut self); 
}

pub struct Template<'a> {
    pub read_position: u32,
    pub position: u32,
    pub input: &'a str,
    pub pattern: Pattern<'a>,
    pub data: Data,
    pub ch : Option<char>,
    pub marks : Vec<Mark>,
    pub tokens : Vec<token::Token>,
}

impl<'a> Template<'a> {
    pub fn new(input: &'a str, data: Data, pattern: Pattern<'a>) -> Self {
        let mut template = Self { 
            input: input,
            pattern: pattern,
            read_position: 0,
            position: 0,
            data : data,
            ch: None,
            marks : vec![],
            tokens : vec![],
        };
        return template
    }

    pub fn read_mark_content(&self, left_index: usize, right_index: usize) -> String { 
        let x = &self.tokens[left_index  .. right_index];
        let content_tokens  = x.iter().filter(|item| {
            item._type != self.pattern.left && item._type != self.pattern.right
        });
        let mut content : String = String::new();
        for token in content_tokens {
            content.push_str(&String::from(&token.literal));
        }
        return content
    }

    pub fn compile(&mut self) -> String {
        self.read();
        let mut mark = Mark::new();
        for (i, item) in self.tokens.iter().enumerate() {
            if item._type == self.pattern.left && mark.left_index == None {
                mark.left_index = Some(i);
            }
            else if item._type == self.pattern.right && mark.right_index == None {
                mark.right_index = Some(i);
            }
            else if mark.left_index != None && mark.right_index != None {
               self.marks.push(mark); 
               mark = Mark::new();
            }
        }

        let mut removed : usize = 0; // Keeps track of removed tokens
        let mut formatted_content : String = String::new(); // Final content tokens
        for (i, item) in self.marks.iter().enumerate() {
            // Grab indexes from mark and decrement to account for removed items
            let mut x : usize = item.left_index.unwrap() - removed;
            let y : usize = item.right_index.unwrap() - removed;
            // Plain text string of the value within a pattern
            let mut data_key : String = self.read_mark_content(x, y);
            // Content from Data map
            let mut content : String = String::new();
            // Check if data_key exists in our self.data (Data) map
            if self.data.data.contains_key(&data_key.to_string()) {
                content.push_str(&self.data.data[&data_key.to_string()]);
            }
            // Keep track of removed item count in this iteration
            let mut temp : usize = 0;
            while x <= y {
                // Remove token at x minus any tokens we just removed
                self.tokens.remove(x - temp);
                temp += 1;
                x += 1;
            }

            // Split and merge self.tokens to include our new content
            let mut section_end: usize = item.left_index.unwrap() - removed;
            let mut left_arr = &self.tokens[ .. section_end].to_vec();                        
            let mut right_arr = &self.tokens[ section_end .. ].to_vec();
            let mut content_token = token::Token::new(token::TEXT, None);
            content_token.literal = content;
            let mut updated_tokens = vec![]; 
            for item in left_arr {
                updated_tokens.push(item.clone());
            }
            updated_tokens.push(content_token);
            for item in right_arr {
                updated_tokens.push(item.clone());
            }
            self.tokens = updated_tokens;
            removed += temp -1;
        }
        
        for item in &self.tokens {
            if item._type != token::EOF {
                formatted_content.push_str(&String::from(&item.literal));
            }
        }
        return formatted_content;
    }
}


impl<'a> Lexer for Template<'a> {

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

#[cfg(test)]
mod tests {
    use super::token;
    use super::Template;
    use super::Data;
    use super::Pattern;
    use super::Lexer;

    
    //#[test]
    fn test_template_create() {
        let st: &str = "my new template {test}";
        let mut d = Data::new();
        let mut t = Template::new(st, d, Pattern::new(token::LEFTBRACKET, token::RIGHTBRACKET));
    }

    #[test]
    fn test_compile() {
        let st: &str = "my {{one}} new template {{two}}";
        let test_string : &str = "my two new template three";
        let mut d = Data::new();
        d.add_many(vec![("one","two"), ("two", "three")]);
        let mut t = Template::new(st, d,  Pattern::new(token::DOUBLELEFTBRACKET, token::DOUBLERIGHTBRACKET));
        let result : String = t.compile();

        //assert_eq!(t.read_position, st.len() as u32);
        assert_eq!(result, test_string);
        assert_eq!(t.input, st);
        assert_eq!(t.data.data.len() as u32, 2); 
    }

    //#[test]
    fn test_read() {
        let st: &str = "my {{one}} {new template {{two}}";
        let mut d = Data::new();
        d.add_many(vec![("one","two"), ("two", "three")]);
        let mut t = Template::new(st, d,  Pattern::new(token::DOUBLELEFTBRACKET, token::DOUBLERIGHTBRACKET));
        let v = vec![
        ("m", token::TEXT),
        ("y", token::TEXT),
        (" ", token::SPACE),
        ("{{", token::DOUBLELEFTBRACKET),
        ("o", token::TEXT),
        ("n", token::TEXT),
        ("e", token::TEXT),
        ("}}", token::DOUBLERIGHTBRACKET),
        (" ", token::SPACE),
        ("{", token::TEXT),
        ("n", token::TEXT),
        ("e", token::TEXT),
        ("w", token::TEXT),
        (" ", token::SPACE),
        ("t", token::TEXT),
        ("e", token::TEXT),
        ("m", token::TEXT),
        ("p", token::TEXT),
        ("l", token::TEXT),
        ("a", token::TEXT),
        ("t", token::TEXT),
        ("e", token::TEXT),
        (" ", token::SPACE),
        ("{{", token::DOUBLELEFTBRACKET),
        ("t", token::TEXT),
        ("w", token::TEXT),
        ("o", token::TEXT),
        ("}}", token::DOUBLERIGHTBRACKET),
        (" ", token::EOF),
        ];
        for test_case in v {
            let n  : token::Token = t.next_token();
            assert_eq!(test_case.0, n.literal);
            assert_eq!(test_case.1, n._type);
        }
    }
}
