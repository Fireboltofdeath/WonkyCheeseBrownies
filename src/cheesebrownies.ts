class Consumer {
	Sauces: String[] = [];

	Eat() {
		console.log("I eat brownie with sauces: " + this.Sauces.toString());
	}

	Sauce(...sauces) {
		sauces.forEach(sauce => {
			console.log("I pour " + sauce + " sauce on my brownie!");
			this.Sauces.push(sauce);
		});
	}

	Cheese() {
		this.Sauce("Cheese");
	}
}