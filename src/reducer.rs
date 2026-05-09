use std::rc::Rc;

use crate::ast::{ Node, IdentMap };

#[derive(Debug, Clone)]
pub struct LazyClosure<'a> {
	node: &'a Node,
	scope: Scope<'a>,
}

impl<'a> LazyClosure<'a> {
	pub fn new(node: &'a Node) -> Self {
		Self {
			node,
			scope: Scope::new(),
		}
	}

	pub fn new_with_scope(node: &'a Node, scope: Scope<'a>) -> Self {
		Self {
			node,
			scope,
		}
	}

	pub fn node(&self) -> &Node { self.node }

	#[allow(dead_code)]
	pub fn scope(&self) -> Scope<'a> { self.scope.clone() }

	pub fn reduce(&self) -> Result<LazyClosure<'a>, ReduceErr> {
		match self.node {
			Node::Apply { lhs, rhs } => {
				let lhs_closure  = LazyClosure::new_with_scope(lhs, self.scope.clone()).reduce()?;
				match lhs_closure.node {
					Node::Func { binding, expr } => LazyClosure::new_with_scope(
						expr,
						lhs_closure.scope.add(
							*binding,
							LazyClosure::new_with_scope(rhs, self.scope.clone())
					)).reduce(),
					_ => Err(ReduceErr::NotFunc)
				}
			},
			Node::Func {..} => {
				Ok(self.clone())
			},
			Node::Ident(ident) => {
				match self.scope.get(*ident) {
					None => Err(ReduceErr::UnknownIdent),
					Some(closure) => closure.reduce()
				}
			}
		}
	}

	// for bindings that have been bound, it needs to recurse
	// for bindings that may have something in scope, but that will have something bound over top of
	// them, those will need to simply be written from the map
	//
	// so in essences, will will need to track, at each call, which bindings there are that have not
	// yet been bound

	pub fn format_with_map(&self, map: &IdentMap, depth: usize, max_depth: usize) -> String {
		let next_depth = depth + 1;

		if depth < max_depth {

		} else {

		}

		match self {
			Node::Func { binding, expr} => {
				let front_pad = "  ".repeat(depth);
				let mut binding_str = try_stringify_ident(&map, *binding);
				binding_str.push_str(" ->\n");
				let binding_line = front_pad + &binding_str;
				let remainder = expr.format_with_map(map, next_depth);
				binding_line + &remainder
			},
			Node::Apply { lhs, rhs} => {
				lhs.format_with_map(map, next_depth) 
				+ &"\n"
				+ &rhs.format_with_map(map, next_depth)
			},
			Node::Ident(ident) => {
				let front_pad = "  ".repeat(depth);
				front_pad + &try_stringify_ident(&map, *ident)
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
pub struct Scope<'a> {
	binding: Option<Rc<ScopeBinding<'a>>>
}

impl<'a> Scope<'a> {
	pub fn new() -> Self {
		Self {
			binding: None
		}
	}

	pub fn get(&self, ident: usize) -> Option<LazyClosure<'a>> {
		match &self.binding {
			Some(binding) => binding.get(ident),
			None => None
		}
	}

	pub fn add(&self, ident: usize, closure: LazyClosure<'a>) -> Self {
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
struct ScopeBinding<'a> {
	ident: usize,
	closure: LazyClosure<'a>,
	prior: Option<Rc<Self>>
}

impl<'a> ScopeBinding<'a> {
	pub fn get(&self, ident: usize) -> Option<LazyClosure<'a>> {
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
