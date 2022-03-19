use crate::{
	parser::Expression,
	tokens::{Operator, Parenthesis, Token},
};

struct Function {}

struct Variable {}

struct Interpreter {}

pub fn execute(expression: &Box<Expression>) -> Option<Token> {
	let mut left: Option<Token> = None;
	let mut right: Option<Token> = None;

	if let Some(exp) = &expression.left {
		left = execute(exp);
	}

	if let Some(exp) = &expression.right {
		right = execute(exp);
	}

	None
}
