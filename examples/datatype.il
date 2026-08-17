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
