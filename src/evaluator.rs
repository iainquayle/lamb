use std::rc::Rc;

use crate::ast::Node;

pub fn reduce(node: &Node) -> Result<LazyClosure, ReduceErr> {
	reduce_rec(Rc::new(node.clone()), &Scope::new())
}

fn reduce_rec(node: Rc<Node>, scope: &Scope) -> Result<LazyClosure, ReduceErr> {
	match node.as_ref() {
		Node::Apply { lhs, rhs } => {
			let lhs_closure = reduce_rec(lhs.clone(), &scope)?;
			match lhs_closure.node.as_ref() {
				Node::Func { binding, expr } => {
					reduce_rec(expr.clone(), &lhs_closure.scope.add(*binding, LazyClosure::new(rhs.clone(), scope.clone())))
				},
				_ => Err(ReduceErr::NotFunc)
			}

		},
		Node::Func {..} => {
			Ok(LazyClosure::new(node.clone(), scope.clone()))
		},
		Node::Ident(ident) => {
			match scope.get(*ident) {
				None => Err(ReduceErr::UnknownIdent),
				Some(closure) => Ok(closure)
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
		Self {
			binding: None
		}
	}

	pub fn get(&self, ident: usize) -> Option<LazyClosure> {
		match &self.binding {
			Some(binding) => binding.get(ident),
			None => None
		}
	}

	pub fn add(&self, ident: usize, closure: LazyClosure) -> Self {
		Scope {
			binding: Some(Rc::new(ScopeBinding {
				ident,
				closure,
				prior: self.binding.clone()
			}))
		}	
	}
}

#[derive(Clone, Debug)]
struct ScopeBinding {
	ident: usize,
	closure: LazyClosure,
	prior: Option<Rc<ScopeBinding>>
}

impl ScopeBinding {
	pub fn get(&self, ident: usize) -> Option<LazyClosure> {
		if self.ident == ident {
			Some(self.closure.clone())
		} else {
			match &self.prior {
				Some(binding) => binding.get(ident),
				None => None
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct LazyClosure {
	node: Rc<Node>,
	scope: Scope,
}

impl LazyClosure {
	pub fn new(node: Rc<Node>, scope: Scope) -> Self {
		Self {
			node,
			scope,
		}
	}

	pub fn node(&self) -> Rc<Node> { self.node.clone() }
	// pub fn scope(&self) -> Scope { self.scope.clone() }
}

