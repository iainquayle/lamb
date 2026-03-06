use std::str::Chars;
use std::iter::Enumerate;

#[derive(Debug)]
pub struct Position {
	pub index: usize
}

#[derive(Debug)]
pub enum TokenType {
	Ident(String),
	ParenOpen,
	ParenClose,
	Assign,
	//Eval,
	Arrow,
	Whitespace,
	Unknown,
}

#[derive(Debug)]
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

macro_rules! ident_pattern {
	() => {
		'a'..'z' | 'A'..'Z' | '_' | '+' | '!' | '|' | '*' | '/' | '&' 
	};
}

macro_rules! whitespace_pattern {
	() => {
		' ' | '\t' | '\n' 
	};
}

/*
 * this seems great but will over shoot on the iter
 */

fn for_ident(src_iter: SrcIter) -> ((String, usize), SrcIter) {
	let mut mut_iter = src_iter.clone();
	let result = mut_iter.by_ref().try_fold((String::new(), 0), | (ident, index), (i, c)| {
		match c {
			ident_pattern!() => Ok((ident + &c.to_string(), i)),
			_ => Err((ident, index))
		}
	});
	match result {
		Ok(acc) => (acc, mut_iter),
		Err(acc) => (acc, mut_iter)
	}
}

fn for_whitespace(src_iter: SrcIter) -> (usize, SrcIter) {
	let mut mut_iter = src_iter.clone();
	let result = mut_iter.by_ref().try_fold(0, |acc, (i, c)| {
		match c {
			whitespace_pattern!() => Ok(i),
			_ => Err(acc)
		}
	});
	match result {
		Ok(acc) => (acc, mut_iter),
		Err(acc) => (acc, mut_iter)
	}
}

pub fn lex(src: String) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut src_iter = src.chars().enumerate();

	while let Some((i, c)) = src_iter.next() {
		let current_iter = src_iter.clone();
		let (token, new_iter) = match c {
			whitespace_pattern!() => {
				let (index, new_iter) = for_whitespace(current_iter);
				if index == 0 {
					(Token::new(index, TokenType::Whitespace), new_iter)
				} else {
					(Token::new(i, TokenType::Whitespace), new_iter)
				}
			},
			'-' => {
				let mut arrow_iter = current_iter.clone();
				match arrow_iter.next() {
					Some((i, '>')) => (Token::new(i, TokenType::Arrow), arrow_iter),
					_ => {
						let ((ident_tail, index), new_iter) = for_ident(current_iter);
						let ident = String::from(c) + ident_tail.as_str();
						(Token::new(index, TokenType::Ident(ident)), new_iter)
					}
				}
			},
			'=' => (Token::new(i, TokenType::Assign), current_iter),
			'(' => (Token::new(i, TokenType::ParenOpen), current_iter),
			')' => (Token::new(i, TokenType::ParenClose), current_iter),
			ident_pattern!() => {
				let ((ident_tail, index), new_iter) = for_ident(current_iter);
				let ident = String::from(c) + ident_tail.as_str();
				(Token::new(index, TokenType::Ident(ident)), new_iter)
			},
			_ => (Token::new(i, TokenType::Unknown), current_iter)
		};
		src_iter = new_iter;
		tokens.push(token);
	}

	tokens
}
