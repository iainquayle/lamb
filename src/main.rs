mod lexer;
mod ast;
mod parser;

use lexer::{lex};
use parser::parse;

fn main() {
    println!("Hello, world!");
	
	let src = String::from("hello -> -h (  )      h");
	let tokens = lex(src);
	println!("{:?}", tokens);

	let ast = parse(tokens);
}
