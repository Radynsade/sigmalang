#[derive(Debug, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Power,
	Equal,
	Factorial,
}

// `(` and `)`
#[derive(Debug, PartialEq)]
pub enum Parenthesis {
	Open,
	Close,
}

// `[` and `]`
pub enum Brackets {
	Open,
	Close
}

// `{` and `}`
pub enum Braces {
	Open,
	Close
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Operator(Operator),
	Keyword(String),
	Parenthesis(Parenthesis),
	Number(f64),
	Identifier(String),
	EOL,
}
