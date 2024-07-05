use crate::token::Token;

enum Precedence {
    Lowest,
    Assign,
    Or,
    And,
    Eq,
    Cmp,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

struct ParseRule {
    prefix: fn(can_assign: bool),
    infix: fn(can_assign: bool),
    precedence: Precedence,
}

struct Parser {
    curr: Token,
    prev: Token,
    had_err: bool,
    panicking: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            
        }
    }
}