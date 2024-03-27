use crate::ast::expr::Expr;
use crate::token::Token;

trait StmtTrait {

}

enum Stmt {
    Assign( Token, Box<Expr> ),
    Block( Token, Box<Expr> ),
    Return( Token, Box<Expr> ),
}