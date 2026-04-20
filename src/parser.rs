use std::rc::Rc;

use crate::ast::{Node, Ast, IdentMap};
use crate::lexer::{Token, TokenType, Position};

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseErrors> {
	let unknown = tokens.clone().into_iter().find(|t| {
		matches!(t, Token {token_type: TokenType::Unknown, ..})
	});
	match unknown {
		Some(token) => Err(ParseErrors::ContainsUnknown(token.position)),
		None => {
			let no_whitespace_tokens: Vec<Token> = tokens.into_iter().filter(|t| { 
				!matches!(t, Token { token_type: TokenType::Whitespace, ..})
			}).collect();
			let mut ident_map = IdentMap::new();
			let result = parse_function(&no_whitespace_tokens, &mut ident_map);
			match result {
				Ok((node, _)) => Ok(Ast {
					map: ident_map,
					node 
				}),
				Err(err) => Err(err)
			}
		}
	}
}

fn parse_function<'a>(tokens: &'a[Token], ident_map: &mut IdentMap) -> Result<(Node, &'a[Token]), ParseErrors> {
	match tokens {
		[ 
			Token { token_type: TokenType::Ident(ident),  .. },
			Token { token_type: TokenType::Arrow, .. }, 
			tokens @ ..
		]  =>  match parse_function(tokens, ident_map) {
			Ok((node, tokens)) => Ok(( 
					Node::Func { 
						binding: ident_map.get_index(ident), 
						expr: Rc::new(node) 
					}, tokens)),
			expr_result @ Err(_) => expr_result
		},
		_ => parse_call(None, tokens, ident_map)
	} 
}

fn parse_call<'a>(lhs: Option<Node>, tokens: &'a[Token], ident_map: &mut IdentMap) -> Result<(Node, &'a[Token]), ParseErrors> {
	match lhs {
		Some(lhs_node) => match parse_primary(tokens, ident_map) {
			Ok((rhs_node, tokens)) => parse_call( 
				Some( Node::Apply {
					lhs: Rc::new(lhs_node), 
					rhs: Rc::new(rhs_node)
				}), tokens, ident_map),
			Err(_) => Ok((lhs_node, tokens))
		} 
		None => match parse_primary(tokens, ident_map) {
			Ok((node, tokens)) => parse_call(Some(node), tokens, ident_map),
			result @ Err(_) => result
		}
	}
}

fn parse_primary<'a>(tokens: &'a[Token], ident_map: &mut IdentMap) -> Result<(Node, &'a[Token]), ParseErrors> {
	match tokens {
		[ Token { token_type: TokenType::ParenOpen, .. }, tokens @ ..] => match parse_function(tokens, ident_map) {
			Ok((node, tokens)) => match tokens {
				[ Token { token_type: TokenType::ParenClose, .. }, tokens @ ..] => Ok((node, tokens)),
				[ Token { position, .. }, ..] => Err(ParseErrors::NoClosingParen(position.clone())),
				[] => Err(ParseErrors::Eof)
			}
			result @ Err(_) => result
		},
		[ Token { token_type: TokenType::Ident(ident), .. }, tokens @ ..] => 
			Ok((Node::Ident(ident_map.get_index(ident)), tokens)),
		[ Token { position, .. }, ..] => Err(ParseErrors::NotPrimary(position.clone())),
		[] => Err(ParseErrors::Eof)
	}
}

#[derive(Debug)]
pub enum ParseErrors {
	#[allow(dead_code)]
	NoClosingParen(Position),	
	#[allow(dead_code)]
	NotPrimary(Position),
	#[allow(dead_code)]
	ContainsUnknown(Position),	
	Eof
}

