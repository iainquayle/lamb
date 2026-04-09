// track scope while applying bindings
// when applying to an ident, replace with scoped binding
// when returning a fn, return it with its scope
// would need to track stack, and when returning a fn, its scope is everything above its binding

use crate::ast::Node;

struct ScopeBinding {
	ident: usize,
	depth: usize,
	node: Node
}



struct Scope {

}

impl Scope {

}

fn reduce_rec(node: Node) -> Node {
	reduce_rec(node, Scope::new())
}

fn reduce_rec(node: Node, scope: Scope) -> Node {
	match node {
		Node::Func { binding, expr } => {

		},
		Node::Apply { lhs, rhs} => {

		},
		Node::Ident(ident) => {

		}
	}

	todo!()
}
