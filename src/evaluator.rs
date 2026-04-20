use std::rc::Rc;

use crate::ast::Node;

#[derive(Debug, Clone)]
pub struct LazyClosure<'a> {
	node: &'a Node,
	scope: Scope,
}

impl<'a> LazyClosure<'a> {
	pub fn new(node: &'a Node) -> LazyClosure<'a> {
		Self {
			node,
			scope: Scope::new(),
		}
	}

	pub fn new_with_scope(node: &'a Node, scope: Scope) -> LazyClosure<'a> {
		Self {
			node,
			scope,
		}
	}

	pub fn node(&self) -> &Node { self.node }
	pub fn scope(&self) -> Scope { self.scope.clone() }

	pub fn reduce(&self) -> Result<LazyClosure, ReduceErr> {
		match self.node {
			Node::Apply { lhs, rhs } => {
				let lhs_closure = LazyClosure::new_with_scope(lhs, self.scope.clone()).reduce()?;
				match lhs_closure.node {
					Node::Func { binding, expr } => LazyClosure::new_with_scope(
						expr,
						lhs_closure.scope.add(
							*binding,
							LazyClosure::new_with_scope(rhs, self.scope.clone())
						)
					).reduce(),
					_ => Err(ReduceErr::NotFunc)
				}

			},
			Node::Func {..} => {
				Ok(LazyClosure::new_with_scope(node.clone(), scope.clone()))
			},
			Node::Ident(ident) => {
				match scope.get(*ident) {
					None => Err(ReduceErr::UnknownIdent),
					Some(closure) => reduce_rec(closure.node(), &closure.scope())
				}
			}
		}
	}
}

// this could be attached to Node?
pub fn reduce(node: &Node) -> Result<LazyClosure, ReduceErr> {
	reduce_rec(Rc::new(node.clone()), &Scope::new())
}

fn reduce_rec(node: Rc<Node>, scope: &Scope) -> Result<LazyClosure, ReduceErr> {
	match node.as_ref() {
		Node::Apply { lhs, rhs } => {
			// technially reduce rec can return something else if it hits an ident that maps to
			// something that isnt a func?
			// may be that reduce ident should actually reduce rather than be lazy
			let lhs_closure = reduce_rec(lhs.clone(), &scope)?;
			match lhs_closure.node.as_ref() {
				Node::Func { binding, expr } => {
					reduce_rec(
						expr.clone(), 
						&lhs_closure.scope.add(
							*binding, 
							LazyClosure::new_with_scope(rhs.clone(), scope.clone())
					))
				},
				_ => Err(ReduceErr::NotFunc)
			}

		},
		Node::Func {..} => {
			Ok(LazyClosure::new_with_scope(node.clone(), scope.clone()))
		},
		Node::Ident(ident) => {
			match scope.get(*ident) {
				None => Err(ReduceErr::UnknownIdent),
				Some(closure) => reduce_rec(closure.node(), &closure.scope())
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
		Self {
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
	prior: Option<Rc<Self>>
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

