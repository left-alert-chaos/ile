# This file holds the code for the Iterator datatype.

datatype Iterator {
	let array = 0;
	let index = 0;

	let next = (self) {
		let index = self.index;
		self.index = self.index + 1;
		#return self.array[self.index];
	};
}

let len = (array) {
	let index = 0;

	while true {
		try {
			let _ = array[index];
		} catch {
			break;
		}

		index = index + 1;
	}

	return index;
};
