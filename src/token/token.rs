enum Token {
	// Misc
	illegal,
	eof,

	// Identifiers and literals
	id,
	bool,
	char,
	string,
	int,
	float,

	// Operators
	assign,
	plus,
	minus,
	star,
	div,
	bang,
	lt,
	gt,
	eq,
	notEq,
	inc,
	dec,

	// Delimiters
	comma,
	semiColon,
	lParen,
	rParen,
	lBrace,
	rBrace,
	arrow,

	// Keywords
	fn,
	var,
	const,
	true,
	false,
	if,
	else,
	switch,
	case,
	return,
	match
}

fn get_token() {

}