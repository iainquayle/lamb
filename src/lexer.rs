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



pub fn lex(src: String) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut token_iter = src.chars().enumerate();

	while let Some((i, c)) = token_iter.next() {
		let token = match c {
			'=' => Token::new(i, TokenType::Assign),
			'(' => Token::new(i, TokenType::ParenOpen),
			')' => Token::new(i, TokenType::ParenClose),
			_ => Token::new(i, TokenType::Unknown) 
		};
		tokens.push(token);
	}

	tokens
}
