use crate::ast::{Node, Ast};
use crate::lexer::{Token, TokenType};

//type TokenIter<'a> = std::slice::Iter<'a, Token>;

pub fn parse(tokens: Vec<Token>) -> Result<Node, String> {
	let filtered_tokens: Vec<Token> = tokens.into_iter().filter(|t| { 
		!matches!(t, Token { token_type: TokenType::Whitespace, ..})
	}).collect();

	//let mut declarations: Vec<Node> = Vec::new();	
	
	let result = parse_function(&filtered_tokens);

	match result {
		Ok((expr, _)) => Ok(expr),
		Err(err) => Err(err)
	}

	
}

fn parse_function(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
	match tokens {
		[ 
			Token { token_type: TokenType::Ident(ident),  .. },
			Token { token_type: TokenType::Arrow, .. }, 
			tokens @ ..
		]  =>  match parse_function(tokens) {
			Ok((node, tokens)) => Ok(( 
					Node::Func { 
						binding: ident.to_string(), 
						expr: Box::new(node) 
					}, tokens)),
			expr_result @ Err(_) => expr_result
		},
		_ => parse_call(None, tokens)
	} 
}

fn parse_call(lhs: Option<Node>, tokens: &[Token]) -> Result<(Node, &[Token]), String> {
	match lhs {
		Some(lhs_node) => match parse_primary(tokens) {
			Ok((rhs_node, tokens)) => parse_call( 
				Some( Node::Call {
					lhs: Box::new(lhs_node), 
					rhs: Box::new(rhs_node)
				}), tokens),
			Err(_) => Ok((lhs_node, tokens))
		} 
		None => match parse_primary(tokens) {
			Ok((node, tokens)) => parse_call(Some(node), tokens),
			result @ Err(_) => result
		}
	}
}

fn parse_primary(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
	match tokens {
		[ Token { token_type: TokenType::ParenOpen, .. }, tokens @ ..] => match parse_function(tokens) {
			Ok((node, tokens)) => match tokens {
				[ Token { token_type: TokenType::ParenClose, .. }, tokens @ ..] => Ok((node, tokens)),
				_ => Err("".to_string())
			}
			result @ Err(_) => result
		},
		[ Token { token_type: TokenType::Ident(ident), .. }, tokens @ ..] => 
			Ok((Node::Ident(ident.to_string()), tokens)),
		_ => Err("".to_string())
	}
}
