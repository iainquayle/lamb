use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ast {
	pub map: IdentMap,
	pub node: Node
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: usize, expr: Rc<Node> },
	Apply { lhs: Rc<Node>, rhs: Rc<Node> },
	Ident(usize)
}

#[derive(Clone, Debug)]
pub struct IdentMap {
	map: HashMap<String, usize>,
	reverse_map: HashMap<usize, String>,
	current_index: usize
}

impl IdentMap {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
			reverse_map: HashMap::new(),
			current_index: 0
		}
	}

	pub fn get_index(&mut self, ident: &String) -> usize {
		match self.map.get(ident) {
			Some(index) => *index,
			None => {
				_ = self.map.insert(ident.clone(), self.current_index);
				_ = self.reverse_map.insert(self.current_index, ident.clone());
				let current_index = self.current_index;
				self.current_index += 1;
				current_index
			}
		}
	}

	pub fn get_ident(&self, index: usize) -> Option<&String> {
		self.reverse_map.get(&index)
	}

	pub fn len(&self) -> usize {
		self.map.len()
	} 
}
