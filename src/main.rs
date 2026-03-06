mod lexer;
mod ast;
mod parser;

use lexer::{lex};

fn main() {
    println!("Hello, world!");
	
	let src = String::from("hello -> -h (  )      h");
	let tokens = lex(src);
	println!("{:?}", tokens)
}
