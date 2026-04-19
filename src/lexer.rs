use std::str::Chars;
use std::iter::Enumerate;

#[derive(Debug, Clone)]
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

fn fold_while<Acc, Fold>(src_iter: SrcIter, acc: Acc, fold: Fold) -> (Acc, usize, SrcIter)
	where
		Fold: Fn(&Acc, char) -> Option<Acc>
{ 
	let mut current_iter = src_iter.clone();
	let mut previous_iter = src_iter.clone();
	let mut found = true;
	let mut current_acc = acc;
	let mut index = 0;
	while let Some((i, c)) = current_iter.next() && found {
		match fold(&current_acc, c) {
			Some(acc) => { current_acc = acc; previous_iter = current_iter.clone(); },
			None => { index = i - 1; found = false; }
		}
	}
	(current_acc, index, previous_iter)
}

fn for_ident(src_iter: SrcIter) -> (String, usize, SrcIter) {
	fold_while(src_iter, String::new(), | acc, c | {
		match c {
			ident_pattern!() => Some(acc.clone() + &c.to_string()),
			_ => None 
		}
	})
}

fn for_whitespace(src_iter: SrcIter) -> (usize, SrcIter) {
	let (_, index, iter) = fold_while(src_iter, (), | _, c | {
		match c {
			whitespace_pattern!() => Some(()),
			_ => None 
		}
	});
	(index, iter)
}

/*
 * !!!!!!!
 * change this so that it just stores the starting index? would be easier
 *
 */

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
						let (ident_tail, index, new_iter) = for_ident(current_iter);
						let ident = String::from(c) + ident_tail.as_str();
						(Token::new(index, TokenType::Ident(ident)), new_iter)
					}
				}
			},
			'=' => (Token::new(i, TokenType::Assign), current_iter),
			'(' => (Token::new(i, TokenType::ParenOpen), current_iter),
			')' => (Token::new(i, TokenType::ParenClose), current_iter),
			ident_pattern!() => {
				let (ident_tail, index, new_iter) = for_ident(current_iter);
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

#[derive(Debug, Clone)]
pub struct Position {
	pub index: usize
}

#[derive(Debug, Clone)]
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

type SrcIter<'a> = Enumerate<Chars<'a>>;

