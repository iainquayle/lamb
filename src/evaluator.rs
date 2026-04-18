// track scope while applying bindings
// when applying to an ident, replace with scoped binding
// when returning a fn, return it with its scope
// would need to track stack, and when returning a fn, its scope is everything above its binding
use std::rc::Rc;

use crate::ast::Node;

pub fn reduce(node: Node) -> Result<(Node, Scope), ReduceErr> {
	reduce_rec(node, None)
}

// the scope for a function is always going to be the same no matter what, even if it gets passed
// back up. really over complicating things, just need a running call stack scope, pass scope back
// up with a fn, and then when evaling that fn, dive back into the scope attached to it.
// maybe i was right though, as what hapens when it get bound to something, then the scope it had
// must follow it around

fn reduce_rec(node: Node, scope: Scope) -> Result<(Node, Scope), ReduceErr> {
	match node {
		Node::Apply { lhs, rhs } => {
			let reduced_lhs = reduce_rec(node, scope);
			todo!()
		},
		Node::Func {..} => {
			Ok((node.clone(), scope))
		},
		Node::Ident(ident) => {
			match scope.get(ident) {
				None => Err(ReduceErr::UnknownIdent),
				Some((node, scope)) => Ok((node.clone(), scope))
			}
		}
	}
}

enum ReduceErr {
	UnknownIdent
}

#[derive(Clone, Debug)]
struct Scope {
	binding: Option<Rc<ScopeBinding>>
}

impl Scope {
	pub fn new() -> Self {
		Scope {
			binding: None
		}
	}

	pub fn get(&self, ident: usize) -> Option<(Node, Scope)> {
		match &self.binding {
			Some(binding) => binding.get(ident),
			None => None
		}
	}

	pub fn add(&self, ident: usize, node: Node, scope: Scope) -> Self {
		Scope {
			binding: Some(Rc::new(ScopeBinding {
				ident: ident,
				node: node.clone(),
				scope: scope.clone(),
				prior: self.binding.clone()
			}))
		}	
	}
}

#[derive(Clone, Debug)]
struct ScopeBinding {
	ident: usize,
	node: Node,
	scope: Scope,
	prior: Option<Rc<ScopeBinding>>
}

impl ScopeBinding {
	pub fn get(&self, ident: usize) -> Option<(Node, Scope)> {
		if self.ident == ident {
			Some((self.node.clone(), self.scope.clone()))
		} else {
			match &self.prior {
				Some(binding) => binding.get(ident),
				None => None
			}
		}
	}
}
