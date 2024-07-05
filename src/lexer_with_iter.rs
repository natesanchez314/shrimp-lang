use std::{iter::Peekable, str::CharIndices};

use crate::token::TokenType;

struct Lexer {
    source: Peekable<std::str::CharIndices<'static>>,
    start: char,
    start_pos: usize,
    curr: char,
    curr_pos: usize,
    line: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut source = input.char_indices().peekable();
        let c = source.next().unwrap();
        Self {
            source,
            start: c.1,
            start_pos: c.0,
            curr: c.1,
            curr_pos: c.0,
            line: 1,
        }
    }
    
    
    fn advance(&mut self) {
        if self.is_at_end() {
            self.curr = '\0';
        } else {
            self.curr = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.curr_pos += 1;
    }

    fn peek(&self) -> char {
        match self.source.peek() {
            Some(c) => {
                c.1
            }
            None => {
                '\0'
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.curr;
        self.start_pos = self.curr_pos;
        match self.curr {
            '(' => { self.new_token(TokenType::LParen, self.curr.to_string()) }
            ')' => { self.new_token(TokenType::RParen, self.curr.to_string()) }
            '{' => { self.new_token(TokenType::LBrace, self.curr.to_string()) }
            '}' => { self.new_token(TokenType::RBrace, self.curr.to_string()) }
            '[' => { self.new_token(TokenType::LBracket, self.curr.to_string()) }
            ']' => { self.new_token(TokenType::RBracket, self.curr.to_string()) }
            ',' => { self.new_token(TokenType::Comma, self.curr.to_string()) }
            ':' => { self.new_token(TokenType::Colon, self.curr.to_string()) }
            ';' => { self.new_token(TokenType::SemiColon, self.curr.to_string()) }
            '.' => { self.new_token(TokenType::Dot, self.curr.to_string()) }
            '+' => { self.new_token(TokenType::Plus, self.curr.to_string()) }
            '-' => { self.new_token(TokenType::Minus, self.curr.to_string()) }
            '*' => { self.new_token(TokenType::Star, self.curr.to_string()) }
            // Div or comment
            '/' => { 
                if self.peek() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()
                    }
                    self.new_token(TokenType::Comment, self.curr.to_string())
                } else {
                    self.new_token(TokenType::Slash, self.curr.to_string())
                }
            }
            // One or two char tokens
            '=' => {
                if self.peek() == '=' {
                    self.new_token(TokenType::EqEq, self.curr.to_string())
                } else {
                    self.new_token(TokenType::Assign, self.curr.to_string())
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.new_token(TokenType::BangEq, self.curr.to_string())
                } else {
                    self.new_token(TokenType::Bang, self.curr.to_string())
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.new_token(TokenType::LessEq, self.curr.to_string())
                } else {
                    self.new_token(TokenType::Less, self.curr.to_string())
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.new_token(TokenType::GreaterEq, self.curr.to_string())
                } else {
                    self.new_token(TokenType::Greater, self.curr.to_string())
                }
            }
            '"' => { self.handle_str() }
            '\0' => { self.new_token(TokenType::Eof, self.curr.to_string()) }
            _ => {
                if is_digit(self.curr) {
                    self.handle_num()
                } else if is_alpha(self.curr) {
                    self.handle_id()
                } else {
                    self.new_token(TokenType::Illegal, self.curr.to_string())
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.curr == '\0'
    }

    fn handle_id(&mut self) -> Token {
    }

    fn handle_str(&mut self) -> Token {
    }

    fn handle_num(&mut self) -> Token {
    }

    fn skip_white_space(&mut self) {
        while  self.peek() == ' ' 
            || self.peek() == '\t' 
            || self.peek() == '\n' 
            || self.peek() == '\r' {
            self.read_char();
        }
    }

    fn new_token(&mut self, token_type: TokenType, curr_char: String) -> Token {

    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
} 

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}