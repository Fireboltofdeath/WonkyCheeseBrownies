use std::cell::RefCell;

struct Consumer {
	sauces: RefCell<Vec<String>>
}

impl Consumer {
	fn new() -> Self {
		Consumer {
			sauces: RefCell::new(Vec::new())
		}
	}
	
	fn eat(&self) {
		println!("I eat brownie with sauces: {:?}", self.sauces.borrow());
	}
	
	fn sauce(&self, sauce: &str) {
		self.sauces.borrow_mut().push(sauce.to_string());
	}
	
	fn cheese(&self) {
		self.sauce("Cheese");
	}
}