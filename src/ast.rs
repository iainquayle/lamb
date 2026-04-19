use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ast {
	pub map: IdentMap,
	pub node: Node
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Func { binding: usize, expr: Rc<Self> },
	Apply { lhs: Rc<Self>, rhs: Rc<Self> },
	Ident(usize)
}

impl Node {
	pub fn format_with_map(&self, map: &IdentMap, depth: usize) -> String {
		let next_depth = depth + 1;
		match self {
			Self::Func { binding, expr} => {
				let front_pad = "  ".repeat(depth);
				let mut binding_str = try_stringify_ident(&map, *binding);
				binding_str.push_str(" ->\n");
				let binding_line = front_pad + &binding_str;
				let remainder = expr.format_with_map(map, next_depth);
				binding_line + &remainder
			},
			// will need parens in here
			Self::Apply { lhs, rhs} => {
				
				lhs.format_with_map(map, next_depth) 
				+ &"\n"
				+ &rhs.format_with_map(map, next_depth)
			},
			Self::Ident(ident) => {
				let front_pad = "  ".repeat(depth);
				front_pad + &try_stringify_ident(&map, *ident)
			}
		}
	}
}

fn try_stringify_ident(map: &IdentMap, index: usize) -> String {
	map.get_ident(index).map_or(format!("Missing: {}", index), |ident| ident.to_string())
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

	pub fn current_index(&self) -> usize {
		self.current_index
	}

	pub fn len(&self) -> usize {
		self.map.len()
	} 
}
