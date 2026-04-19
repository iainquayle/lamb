use std::str::Chars;
use std::iter::Enumerate;

#[derive(Debug, Clone)]
pub struct Token {
	pub token_type: TokenType,	
	pub position: Position,
}

impl Token {
	pub fn new(token_type: TokenType, position: Position) -> Token {
		Token {
			token_type,
			position,
		}
	}
}

macro_rules! ident_pattern {
	() => {
		'a'..'z' | 'A'..'Z' | '0'..'9' | '_' | '+' | '!' | '|' | '*' | '/' | '&' 
	};
}

macro_rules! whitespace_pattern {
	() => {
		' ' | '\t' | '\n' 
	};
}

pub fn lex(src: String) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut src_iter = src.chars().enumerate();

	let mut current_position = Position::new();
	let mut prev_position = Position::new();

	while let Some((_, c)) = src_iter.next() {
		let current_iter = src_iter.clone();
		current_position.increment(c);
		let (token, new_iter) = match c {
			whitespace_pattern!() => {
				let (new_iter, position) = for_whitespace(current_iter, current_position.clone());
				current_position = position;
				(Token::new(TokenType::Whitespace, prev_position), new_iter)
			},
			'-' => {
				let mut arrow_iter = current_iter.clone();
				match arrow_iter.next() {
					Some((_, '>')) => { 
						current_position.increment('>');
						(Token::new(TokenType::Arrow, prev_position), arrow_iter) 
					},
					_ => {
						let (ident_tail, new_iter, position) = for_ident(current_iter, current_position.clone());
						current_position = position;
						let ident = String::from(c) + &ident_tail;
						(Token::new(TokenType::Ident(ident), prev_position), new_iter)
					}
				}
			},
			'=' => (Token::new(TokenType::Assign, prev_position), current_iter),
			'(' => (Token::new(TokenType::ParenOpen, prev_position), current_iter),
			')' => (Token::new(TokenType::ParenClose, prev_position), current_iter),
			ident_pattern!() => {
				let (ident_tail, new_iter, position) = for_ident(current_iter, current_position.clone());
				current_position = position;
				let ident = String::from(c) + &ident_tail;
				(Token::new(TokenType::Ident(ident), prev_position), new_iter)
			},
			_ => (Token::new(TokenType::Unknown, prev_position), current_iter)
		};
		prev_position = current_position.clone();
		src_iter = new_iter;
		tokens.push(token);
	}

	tokens
}

#[derive(Debug, Clone)]
pub struct Position {
	pub index: usize,
	pub line: usize,
	pub col: usize
}

impl Position {
	pub fn new() -> Self {
		Self {
			index: 0, 
			line: 0, 
			col: 0
		}
	}

	pub fn increment(&mut self, c: char) {
		match c {
			'\n' | '\r' => {
				self.index += 1;
				self.line += 1;
				self.col = 0;
			},
			_ => {
				self.index += 1;
				self.col += 1;
			}
		};
	}
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

fn fold_while<Acc, Fold>(src_iter: SrcIter, acc: Acc, position: Position, fold: Fold) -> (Acc, SrcIter, Position)
	where
		Fold: Fn(&Acc, char) -> Option<Acc>
{ 
	let mut current_position = position;
	let mut current_iter = src_iter.clone();
	let mut previous_iter = src_iter.clone();
	let mut found = true;
	let mut current_acc = acc;
	while let Some((_, c)) = current_iter.next() && found {
		match fold(&current_acc, c) {
			Some(acc) => { 
				current_acc = acc; 
				previous_iter = current_iter.clone(); 
				current_position.increment(c);	
			},
			None => { found = false; }
		}
	}
	(current_acc, previous_iter, current_position)
}

fn for_ident(src_iter: SrcIter, position: Position) -> (String, SrcIter, Position) {
	fold_while(src_iter, String::new(), position,| acc, c | {
		match c {
			ident_pattern!() => Some(acc.clone() + &c.to_string()),
			_ => None 
		}
	})
}

fn for_whitespace(src_iter: SrcIter, position: Position) -> (SrcIter, Position) {
	let (_, iter, position) = fold_while(src_iter, (), position,| _, c | {
		match c {
			whitespace_pattern!() => Some(()),
			_ => None 
		}
	});
	(iter, position)
}

