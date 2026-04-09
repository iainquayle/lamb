// perhaps change this such that it can immediately start being used for eval?
// would likely want to move to rcs, and if wanting to use tracked scope, have a scope attachment
// to fns?

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: usize, expr: Rc<Node> },
	Apply { lhs: Rc<Node>, rhs: Rc<Node> },
	Ident(usize)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
	pub max_ident: usize,
	pub node: Node
}
