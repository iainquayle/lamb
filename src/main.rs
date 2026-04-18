mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::lex;
use parser::parse;
use evaluator::reduce;

fn main() {
	let src = String::from("(t -> f -> t) (x -> x) (y -> y)");
	let tokens = lex(src);
	println!("{:?}\n", tokens);

	let parse_result = parse(tokens);
	match parse_result {
		Ok(ast) => {
			println!("{:?}\n", ast);

			match reduce(&ast.node) {
				Ok(closure) => {
					println!("{:?}\n", closure.node());
				},
				Err(err) => {
					println!("{:?}", err);
				}
			}
		},
		Err(err) => println!("failed to parse {:?}", err)
	}
}
