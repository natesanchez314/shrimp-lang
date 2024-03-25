mod expr;
mod stmt;

trait Node {
    fn TokenLiteral() -> String;
    fn ToString() -> String;
}