use std::collections::HashMap;

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    curr_char: char,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(input: String) -> Self {
        println!("{}", input);
        //let c = input.chars().nth(0).unwrap();
        let mut l = Self {
            input,
            pos: 0,
            read_pos: 0,
            curr_char: '\0',
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
        };
        l.read_char();
        l
    }

    pub fn scan(&mut self) -> Vec<Token> { 
        let mut tokens = Vec::<Token>::new();
        loop {
            let token = self.next_token();
            if token.token_type == TokenType::Eof || token.token_type == TokenType::Illegal {
                break;
            }
            tokens.push(token);
        }
        tokens.push(Token{token_type: TokenType::Eof, literal: "".to_string()});

        tokens
    }
    
    fn read_char(&mut self) {
        if self.is_at_end() {
            self.curr_char = '\0';
        } else {
            self.curr_char = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            match self.input.chars().nth(self.read_pos) {
                Some(c) => c,
                None => '\0'
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        match self.curr_char {
            '(' => { self.new_token(TokenType::LParen, self.curr_char.to_string()) }
            ')' => { self.new_token(TokenType::RParen, self.curr_char.to_string()) }
            '{' => { self.new_token(TokenType::LBrace, self.curr_char.to_string()) }
            '}' => { self.new_token(TokenType::RBrace, self.curr_char.to_string()) }
            '[' => { self.new_token(TokenType::LBracket, self.curr_char.to_string()) }
            ']' => { self.new_token(TokenType::RBracket, self.curr_char.to_string()) }
            ',' => { self.new_token(TokenType::Comma, self.curr_char.to_string()) }
            ':' => { self.new_token(TokenType::Colon, self.curr_char.to_string()) }
            ';' => { self.new_token(TokenType::SemiColon, self.curr_char.to_string()) }
            '.' => { self.new_token(TokenType::Dot, self.curr_char.to_string()) }
            '+' => { self.new_token(TokenType::Plus, self.curr_char.to_string()) }
            '-' => { self.new_token(TokenType::Minus, self.curr_char.to_string()) }
            '*' => { self.new_token(TokenType::Star, self.curr_char.to_string()) }
            // Div or comment
            '/' => { 
                if self.peek_char() == '/' {
                    while self.peek_char() != '\n' && !self.is_at_end() {
                        self.read_char()
                    }
                    self.new_token(TokenType::Comment, self.curr_char.to_string())
                } else {
                    self.new_token(TokenType::Slash, self.curr_char.to_string())
                }
            }
            // One or two char tokens
            '=' => {
                if self.peek_char() == '=' {
                    self.new_token(TokenType::EqEq, self.curr_char.to_string())
                } else {
                    self.new_token(TokenType::Assign, self.curr_char.to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.new_token(TokenType::BangEq, self.curr_char.to_string())
                } else {
                    self.new_token(TokenType::Bang, self.curr_char.to_string())
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.new_token(TokenType::LessEq, self.curr_char.to_string())
                } else {
                    self.new_token(TokenType::Less, self.curr_char.to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.new_token(TokenType::GreaterEq, self.curr_char.to_string())
                } else {
                    self.new_token(TokenType::Greater, self.curr_char.to_string())
                }
            }
            '"' => { self.handle_str() }
            '\0' => { self.new_token(TokenType::Eof, self.curr_char.to_string()) }
            _ => {
                if is_digit(self.curr_char) {
                    self.handle_num()
                } else if is_letter(self.curr_char) {
                    self.handle_id()
                } else {
                    self.new_token(TokenType::Illegal, self.curr_char.to_string())
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.read_pos >= self.input.len()
    }

    fn handle_id(&mut self) -> Token {
        let mut lit = String::new();
        lit.push(self.curr_char);
        while is_alpha_numeric(self.peek_char()) {
            self.read_char();
            lit.push(self.curr_char);
        }
        if self.keywords.contains_key(&lit) {
            self.new_token(self.keywords.get(&lit).unwrap().clone(), lit)
        } else {
            self.new_token(TokenType::Id, lit)
        }
    }

    fn handle_str(&mut self) -> Token {
        let mut lit = String::new();
        while self.peek_char() != '"' && !self.is_at_end() {
            self.read_char();
            lit.push(self.curr_char);
        }
        self.read_char();
        self.new_token(TokenType::String, lit)
    }

    fn handle_num(&mut self) -> Token {
        let mut is_float = false;
        let mut lit = String::new();
        lit.push(self.curr_char);
        while is_digit(self.peek_char()) {
            self.read_char();
            lit.push(self.curr_char);
        }
        if self.peek_char() == '.' {
            is_float = true;
            self.read_char();
            lit.push(self.curr_char);
            while is_digit(self.peek_char()) {
                self.read_char();
                lit.push(self.curr_char);
            }
        }
        if is_float {
            self.new_token(TokenType::Float, lit)
        } else {
            self.new_token(TokenType::Int, lit)
        }
    }

    fn skip_white_space(&mut self) {
        while self.curr_char == ' ' || self.curr_char == '\t' || self.curr_char == '\n' || self.curr_char == '\r' {
            self.read_char();
        }
    }

    fn new_token(&mut self, token_type: TokenType, curr_char: String) -> Token {
        self.read_char();
        Token {
            token_type,
            literal: curr_char,
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