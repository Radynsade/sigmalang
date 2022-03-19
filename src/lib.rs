#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}

mod interpreter;
mod lexer;
mod parser;
pub mod tokens;

pub fn execute(text: &String) {
	// let tokens: Vec<tokens::Token> = lexer::tokenize(&text);

	// for token in tokens.into_iter() {
	// 	println!("{:?}", token);
	// }
}
