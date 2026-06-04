mod lexer;
mod ast;
mod parser;
mod reducer;

use lexer::lex;
use parser::parse;
use reducer::LazyClosure;

fn main() {
	let src = String::from(include_str!("../examples/std.lamb"));
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
					println!("{:?}\n", closure.scope());
					println!("{}\n", closure.format_with_map(&ast.map, 0, 30));
				},
				Err(err) => {
					println!("{:?}", err);
				}
			}
		},
		Err(err) => println!("failed to parse {:?}", err)
	}
}
