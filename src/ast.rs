pub enum Node {
	Func { binding: String, output: Box<Node> },
	Call(Box<Node>),
	Ident(String)
}

pub struct Ast {
	pub declarations: Vec<Node>,
	pub eval: Node
}
