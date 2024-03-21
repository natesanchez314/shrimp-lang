#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
	// Misc
	_Illegal, Eof,

	// Identifiers and literals
	Id, String, Int, Float,

	// Operators
	Assign, Plus, Minus, Star, Slash,
	Less, LessEq, Greater, GreaterEq, EqEq, BangEq,
	Bang,

	// Delimiters
	Dot, Comma, SemiColon,
	LParen, RParen, LBrace, RBrace, LBracket, RBracket,

	// Keywords
	Fn, Var, Const, Struct, 
	True, False,
	If, Else,
	While, For,
	This, Return, Nil,
	And, Or, 
}


pub(crate) struct Token {
	pub(crate) token_type: TokenType,
	pub(crate) lexeme: String,
	pub(crate) literal: Option<String>,
	pub(crate) line: usize,
}

impl ToString for Token {
	fn to_string(&self) -> String {
		format!("Token: {{ type: {:?}, lexeme: {}, literal: {:?} }}", self.token_type, self.lexeme, self.literal)
	}
}