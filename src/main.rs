mod lexer;

use lexer::{lex};

fn main() {
    println!("Hello, world!");
	
	let src = String::from("hello");
	let tokens = lex(src);
	println!("{}", tokens.len())
}
