#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: String, expr: Box<Node> },
	Call { lhs: Box<Node>, rhs: Box<Node> },
	Ident(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
	pub declarations: Vec<Node>,
	pub eval: Node
}
