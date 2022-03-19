use std::collections::VecDeque;

use crate::tokens::*;

pub struct TokensQueue {
	tokens: VecDeque<Token>
}

impl TokensQueue {
	pub fn new(tokens: VecDeque<Token>) -> Self {
		TokensQueue {
			tokens
		}
	}

	pub fn push(&self, token: Token) {
		self.tokens.push_back(token);
	}

	pub fn peek(&self) -> Option<Token> {
		self.tokens.pop_front()
	}

	pub fn next(&self) -> Option<&Token> {
		self.tokens.get(1)
	}

	pub fn len(&self) -> usize {
		self.tokens.len()
	}

	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}
}

fn get_operator_type(c: char) -> Result<Operator, bool> {
	if c == '+' {
		return Ok(Operator::Plus);
	}

	if c == '-' {
		return Ok(Operator::Minus);
	}

	if c == '*' {
		return Ok(Operator::Multiply);
	}

	if c == '/' {
		return Ok(Operator::Divide);
	}

	if c == '^' {
		return Ok(Operator::Power);
	}

	if c == '!' {
		return Ok(Operator::Factorial);
	}

	if c == '=' {
		return Ok(Operator::Equal);
	}

	Err(false)
}

fn get_parenthesis_type(c: char) -> Result<Parenthesis, bool> {
	if c == '(' {
		return Ok(Parenthesis::Open);
	}

	if c == ')' {
		return Ok(Parenthesis::Close);
	}

	Err(false)
}

fn define_token(word: &String) -> Token {
	match word.parse::<f64>() {
		Ok(value) => Token::Number(value),
		Err(_) => panic!("Cannot define token '{}'!", word),
	}
}

pub fn tokenize(text: &String) -> TokensQueue {
	let mut tokens: VecDeque<Token> = vec![];
	let mut word: String = String::from("");

	for c in text.chars() {
		if matches!(c, ' ' | '\n') {
			if !word.is_empty() {
				tokens.push_back(define_token(&word));
				word.clear();
			}

			continue;
		}

		match get_operator_type(c) {
			Ok(operator_type) => {
				if !word.is_empty() {
					tokens.push_back(define_token(&word));
					word.clear();
				}

				tokens.push_back(Token::Operator(operator_type));
				word.clear();
				continue;
			}
			Err(_) => (),
		};

		match get_parenthesis_type(c) {
			Ok(parenthesis_type) => {
				if !word.is_empty() {
					tokens.push_back(define_token(&word));
					word.clear();
				}

				tokens.push_back(Token::Parenthesis(parenthesis_type));
				word.clear();
				continue;
			}
			Err(_) => (),
		}

		word.push(c);
	}

	TokensQueue::new(tokens)
}
