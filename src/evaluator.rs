use crate::ast::{Node};

// track scope while applying bindings
// when applying to an ident, replace with scoped binding
// when returning a fn, return it with its scope
// would need to track stack, and when returning a fn, its scope is everything above its binding

// should use more RCs
struct Scope {
	bindings: Vec<Vec<(Node, usize)>> // id, node, scope level
}

// on call, record scope

impl Scope {
	pub fn new(max_id: usize) -> Self {
		Self { bindings: Vec::new() }
	}

	pub fn push(id: usize, node: Node, level: usize) {

	}

	pub fn fetch(id: usize) {

	}
}

pub fn evaluate(node: Node) {
	todo!()
}
