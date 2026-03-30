// perhaps change this such that it can immediately start being used for eval?
// would likely want to move to rcs, and if wanting to use tracked scope, have a scope attachment
// to fns?

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: usize, expr: Box<Node> },
	Apply { lhs: Box<Node>, rhs: Box<Node> },
	Ident(usize)
}
