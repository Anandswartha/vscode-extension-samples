// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
	// the name of the world in the `*.wit` input file
	world: "calculator",
});

use exports::vscode::example::types::Guest;

use crate::exports::vscode::example::types::{ GuestEngine, Operation };

struct CalcEngine {
	left: u32,
	right: u32,
	operation: Operation,
}

impl GuestEngine for CalcEngine {

	fn new(left: u32, right: u32, operation: Operation) -> Self {
		Self { left, right, operation }
	}

	fn execute(&self) -> u32 {
		match self.operation {
			Operation::Add => self.left + self.right,
			Operation::Sub => self.left - self.right,
			Operation::Mul => self.left * self.right,
			Operation::Div => self.left / self.right,
		}
	}
}

struct Implementation;
impl Guest for Implementation {
	type Engine = CalcEngine;
}

export!(Implementation);