# This file holds the code for the Iterator datatype.

datatype Iterator {
	let array = 0;
	let index = 0;

	let next = (self) {
		let index = self.index;
		self.index = self.index + 1;

		if index >= len(self.array) - 1 {
			return;
		}

		return self.array[self.index];
	};
}

let new_iterator = (array) {
	let iter = Iterator();
	iter.array = array;
	return iter;
};

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
