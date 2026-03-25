#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: usize, expr: Box<Node> },
	Apply { lhs: Box<Node>, rhs: Box<Node> },
	Ident(usize)
}
