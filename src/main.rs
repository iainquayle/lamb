mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::{lex};
use parser::parse;

fn main() {
	let src = String::from("(x -> y -> x ( x y ) y)");
	let tokens = lex(src);
	println!("{:?}", tokens);

	let ast = parse(tokens);
	println!("{:?}", ast);
}
