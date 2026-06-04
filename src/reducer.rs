use std::rc::Rc;
use std::collections::hash_set::HashSet;

use crate::ast::{ Node, IdentMap };

/*
 *
 * ident mapping + side effects
 *
 * hypothetically if an expression pulled in through a side effect used the same int idents despite
 * not being the same string, so long as the systems binding stacks remained correct.
 *
 * and as far as i can tell, there would be no issue assigning idents based on expression depth,
 * it could likely be trimmed even further based on depth and possible scope overwrite, but in most
 * cases it wouldnt provide gains.
 *
 */

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
	// them, those will need to simply be written from the ident map
	//
	// so in essences, will need to track, at each call, which bindings there are that have not
	// yet been bound
	//
	// this also needs to actually be split into two seperate fns, one that starts at the top of the
	// lazy closure, but then also be able to explore the rest of a fn in the context of that
	// closure.

	pub fn format_with_map(&self, ident_map: &IdentMap, depth: usize, max_depth: usize) -> String {
		let next_depth = depth + 1;
		self.format_with_map_rec(self.node, ident_map, HashSet::new(), next_depth, max_depth)
	}

	fn format_with_map_rec(&self, node: &Node, ident_map: &IdentMap, unbound_idents: HashSet<usize>, depth: usize, max_depth: usize) -> String {
		let next_depth = depth + 1;
		match node {
			Node::Func { binding, expr} => {
				let front_pad = "  ".repeat(depth);
				let mut binding_str = try_stringify_ident(&ident_map, *binding);
				binding_str.push_str(" ->\n");
				let binding_line = front_pad + &binding_str;
				let mut unbound_idents = unbound_idents.clone();
				unbound_idents.insert(*binding);
				let remainder = self.format_with_map_rec(expr, ident_map, unbound_idents, next_depth, max_depth);
				binding_line + &remainder
			},
			Node::Apply { lhs, rhs} => {
				self.format_with_map_rec(lhs, ident_map, unbound_idents, next_depth, max_depth)
				+ &"\n"
				+ &self.format_with_map_rec(lhs, ident_map, unbound_idents, next_depth, max_depth)
			},
			Node::Ident(ident) => {
				let front_pad = "  ".repeat(depth);

				match (unbound_idents.contains(ident), self.scope.get(*ident)) {
					(true, _) | (_, None) => front_pad + &try_stringify_ident(&ident_map, *ident),
					(false, Some(closure)) => {

					}
				}

			}
		}
	}
}

fn try_stringify_ident(ident_map: &IdentMap, index: usize) -> String {
	ident_map.get_ident(index).map_or(format!("Missing: {}", index), |ident| ident.to_string())
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
