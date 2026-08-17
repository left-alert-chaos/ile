# This file defines a datatype, which is like a template for a data object.

datatype MyType {
	let x = 5;
	let method = (self) {
		self.modifier();
		return self.x;
	};
	let modifier = (self) {
		self.x = 7;
		let self.y = 0;
	};
}

let instance = MyType();
let seven = instance.method();
