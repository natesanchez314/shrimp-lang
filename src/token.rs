#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TokenType {
	// Misc
	Illegal, Comment, Eof,

	// Identifiers and literals
	Id, String, Int, Float,

	// Operators
	Assign, Plus, Minus, Star, Slash,
	Less, LessEq, Greater, GreaterEq, EqEq, BangEq,
	Bang,

	// Delimiters
	Dot, Comma, Colon, SemiColon,
	LParen, RParen, LBrace, RBrace, LBracket, RBracket,

	// Keywords
	Fn, Var, Const, Struct, 
	True, False,
	If, Else,
	While, For,
	This, Return, Nil,
	And, Or, 
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
	pub(crate) token_type: TokenType,
	pub(crate) literal: String,
}

impl ToString for Token {
	fn to_string(&self) -> String {
		format!("Token: {{ type: {:?}, literal: {:?} }}", self.token_type, self.literal)
	}
}