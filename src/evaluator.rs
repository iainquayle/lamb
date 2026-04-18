use std::rc::Rc;

use crate::ast::Node;

pub fn reduce(node: &Node) -> Result<(Node, Scope), ReduceErr> {
	reduce_rec(node, &Scope::new())
}

fn reduce_rec(node: &Node, scope: &Scope) -> Result<(Node, Scope), ReduceErr> {
	match node {
		Node::Apply { lhs, rhs } => {
			let (reduced_lhs, reduced_scope) = reduce_rec(lhs, &scope)?;
			match reduced_lhs {
				Node::Func { binding, expr } => {
					reduce_rec(&expr, &reduced_scope.add(binding, rhs, scope))
				},
				_ => Err(ReduceErr::NotFunc)
			}

		},
		Node::Func {..} => {
			Ok((node.clone(), scope.clone()))
		},
		Node::Ident(ident) => {
			match scope.get(*ident) {
				None => Err(ReduceErr::UnknownIdent),
				Some((node, scope)) => Ok((node.clone(), scope))
			}
		}
	}
}

#[derive(Debug)]
pub enum ReduceErr {
	UnknownIdent,
	NotFunc,
}

#[derive(Clone, Debug)]
pub struct Scope {
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

	pub fn add(&self, ident: usize, node: &Node, scope: &Scope) -> Self {
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
