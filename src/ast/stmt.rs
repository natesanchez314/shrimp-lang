use crate::ast::expr::Expr;
use crate::token::Token;

use super::Node;

trait StmtTrait {

}

pub enum Stmt {
    Assign( Token, Box<Expr> ),
    Block( Token, Box<Expr> ),
    Expr,
    Let,
    Return( Token, Box<Expr> ),
}

impl Stmt {

}

impl Node for Stmt {
    fn TokenLiteral() -> String {
        todo!()
    }

    fn ToString() -> String {
        todo!()
    }
}