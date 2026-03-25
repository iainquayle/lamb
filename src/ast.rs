#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: String, expr: Box<Node> },
	Call { lhs: Box<Node>, rhs: Box<Node> },
	Ident(String)
}
