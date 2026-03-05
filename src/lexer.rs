use std::str::Chars;
use std::iter::Enumerate;

pub struct Position {
	pub index: usize
}

pub enum TokenType {
	Ident(String),
	ParenOpen,
	ParenClose,
	Assign,
	Eval,
	Arrow,
	Whitespace,
	Unknown,
}

pub struct Token {
	pub position: Position,
	pub token_type: TokenType,	
}

impl Token {
	pub fn new(index: usize, token_type: TokenType) -> Token {
		Token {
			position: Position { index },
			token_type
		}
	}
}

type SrcIter<'a> = Enumerate<Chars<'a>>;

fn in_ident(src: SrcIter) -> (String, SrcIter) { 

}

pub fn lex(src: String) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut src_iter = src.chars().enumerate();

	while let Some((i, c)) = src_iter.next() {
		let current_iter = src_iter.clone();
		let (token, new_iter) = match c {
			'-' => {
				let mut arrow_iter = current_iter.clone();
				match arrow_iter.next() {
					Some((i, '>')) => (Token::new(i, TokenType::Arrow), arrow_iter),
					_ => (Token::new(i, TokenType::Ident(String::from("-"))), current_iter)
						// if this is ident it would need to see how far it can go
				}
			}
			'=' => (Token::new(i, TokenType::Assign), current_iter),
			'(' => (Token::new(i, TokenType::ParenOpen), current_iter),
			')' => (Token::new(i, TokenType::ParenClose), current_iter),
			_ => (Token::new(i, TokenType::Unknown), current_iter)
		};
		src_iter = new_iter;
		tokens.push(token);
	}

	tokens
}
