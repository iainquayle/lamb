use crate::ast::{Node};

// should use more RCs
struct Scope {
	bindings: Vec<Vec<(Node, usize)>> // id, node, scope level
}

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
