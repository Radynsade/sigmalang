use crate::{
	tokens::{
		Operator,
		Token
	},
	lexer::TokensQueue
};

pub type ChildExpression = Option<Box<Expression>>;

pub struct Expression {
	pub token: Token,
	pub left: ChildExpression,
	pub right: ChildExpression,
}

impl Expression {
	fn new(
		token: Token,
		left: Expression,
		right: Expression,
	) -> Self {
		Expression {
			token,
			left: Some(Box::new(left)),
			right: Some(Box::new(right)),
		}
	}

	fn primitive(token: Token) -> Self {
		Expression {
			token,
			left: None,
			right: None
		}
	}
}

pub fn parse(tokens: &TokensQueue) -> Expression {
	let len: usize = tokens.len();
	let mut left: Option<Expression> = None;
	let mut right: Option<Expression> = None;
	let mut op: Option<Token> = None;

	while let Some(token) = tokens.peek() {
		match token {
			Token::Operator(operator) => {
				let next = tokens.next();
			},
			Token::Number(value) => {

			},
		}
	}

	// // while .peek()
	// for _ in 0..len {
	// 	token = tokens.peek();

	// 	if let Some(t) = token {
	// 		match t {
	// 			Token::Operator(operator) => {
	// 				let next = tokens.next();
	// 			},
	// 			Token::Number(value) => {

	// 			},
	// 		}
	// 	}
	// }

	// for token in tokens.into_iter() {
	// 	match token {
	// 		Token::Operator(operator) => {
	// 			token;
	// 		},
	// 		Token::Number(value) => {
	// 			if let None = left {
	// 				left = Some(Expression::primitive(token));
	// 				continue;
	// 			}

	// 			if let Some(_) = op {
	// 				right = Some(Expression::primitive(token));
	// 				continue;
	// 			}

	// 			panic!("You must specify an operator between operands!");
	// 		},
	// 	};
	// }

	Expression {
		token: Token::EOL,
		left: None,
		right: None,
	}
}
