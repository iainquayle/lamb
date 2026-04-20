mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::lex;
use parser::parse;
use evaluator::LazyClosure;

fn main() {
	// let src = String::from("(t -> f -> t) (x -> x) (y -> y)");
	let src = String::from(include_str!("../examples/bool.lamb"));
	let tokens = lex(src);
	println!("{:?}\n", tokens);
	let parse_result = parse(tokens);
	match parse_result {
		Ok(ast) => {
			println!("{:?}\n", ast);
			println!("{}\n", ast.node.format_with_map(&ast.map, 0));
			match LazyClosure::new(&ast.node).reduce() {
				Ok(closure) => {
					println!("{:?}\n", closure.node());
					println!("{}\n", closure.node().format_with_map(&ast.map, 0));
				},
				Err(err) => {
					println!("{:?}", err);
				}
			}
		},
		Err(err) => println!("failed to parse {:?}", err)
	}
}
