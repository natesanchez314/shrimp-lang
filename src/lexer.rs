use std::collections::HashMap;

use crate::token::{self, Token, TokenType};

pub(crate) struct Lexer {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}


impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
            keywords: HashMap::from([
                ("var".to_owned(), TokenType::Var),
                ("const".to_owned(), TokenType::Const),
                ("fn".to_owned(), TokenType::Fn),
                ("struct".to_owned(), TokenType::Struct),
                ("if".to_owned(), TokenType::If),
                ("else".to_owned(), TokenType::Else),
                ("for".to_owned(), TokenType::For),
                ("while".to_owned(), TokenType::While),
                ("return".to_owned(), TokenType::Return),
                ("this".to_owned(), TokenType::This),
                ("true".to_owned(), TokenType::True),
                ("false".to_owned(), TokenType::False),
                ("and".to_owned(), TokenType::And),
                ("or".to_owned(), TokenType::Or),
                ("nil".to_owned(), TokenType::Nil),
            ])
        }
    }

    pub(crate) fn eval(&mut self) -> &Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.eval_token();
        }
        let _ = &self.tokens.push(Token{ 
            token_type: TokenType::Eof, 
            lexeme: "".to_owned(),
            literal: None, 
            line: self.line, 
        });
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.input.len()
    }

    fn eval_token(&mut self) {
        let c = self.next_token();
        match c {
            '(' => { self.add_token(TokenType::LParen, None) }
            ')' => { self.add_token(TokenType::RParen, None) }
            '{' => { self.add_token(TokenType::LBrace, None) }
            '}' => { self.add_token(TokenType::RBrace, None) }
            '[' => { self.add_token(TokenType::LBracket, None) }
            ']' => { self.add_token(TokenType::RBracket, None) }
            ',' => { self.add_token(TokenType::Comma, None) }
            ';' => { self.add_token(TokenType::SemiColon, None) }
            '.' => { self.add_token(TokenType::Dot, None) }
            '+' => { self.add_token(TokenType::Plus, None) }
            '-' => { self.add_token(TokenType::Minus, None) }
            '*' => { self.add_token(TokenType::Star, None) }
            // Div or comment
            '/' => { 
                if self.matches_expected('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next_token();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            // One or two char tokens
            '=' => {
                if self.matches_expected('=') {
                    self.add_token(TokenType::EqEq, None)
                } else {
                    self.add_token(TokenType::Assign, None)
                }
            }
            '!' => {
                if self.matches_expected('=') {
                    self.add_token(TokenType::BangEq, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            '<' => {
                if self.matches_expected('=') {
                    self.add_token(TokenType::LessEq, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            }
            '>' => {
                if self.matches_expected('=') {
                    self.add_token(TokenType::GreaterEq, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            }
            // Skip white space
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => { self.line += 1 }
            '"' => { self.handle_string() }
            _ => {
                if is_digit(c) {
                    self.handle_number();
                } else if is_letter(c) {
                    self.handle_identifier()
                } else {
                    // Error!
                }
            }
        }
    }

    fn next_token(&mut self) -> char {
        let c = self.input.chars().nth(self.curr).unwrap();
        self.curr += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.input.chars().nth(self.curr).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.curr + 1 >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(self.curr + 1).unwrap()
    }

    fn matches_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } 
        if self.input.chars().nth(self.curr).unwrap() != expected {
            return false;
        }
        self.curr += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        self.tokens.push(Token{
            token_type,
            lexeme: self.input[self.start..self.curr].to_string(), 
            literal,
            line: self.line,
        })
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next_token();
            if self.is_at_end() {
                //err
                return;
            }
        }
        self.next_token();
        let val = self.input[self.start + 1..self.curr - 1].to_owned();
        self.add_token(TokenType::String, Some(val));
    }

    fn handle_number(&mut self) {
        let mut is_float = false;
        while is_digit(self.peek()) { self.next_token(); }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            is_float = true;
            self.next_token();
            while is_digit(self.peek()) { self.next_token(); }
        }
        let val = self.input[self.start..self.curr].to_owned();
        if is_float {
            self.add_token(TokenType::Float, Some(val))
        } else {
            self.add_token(TokenType::Int, Some(val))
        }
    }

    fn handle_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.next_token();
        }
        let text = &self.input[self.start..self.curr];
        let token_type = self.keywords.get(text);
        match token_type {
            Some(t) => {
                self.add_token(t.clone(), None)
            }
            None =>{
                self.add_token(TokenType::Id, None)
            }
        }
    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_letter(c) || is_digit(c)
}

fn is_letter(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
} 

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}