# This file holds the code for the Iterator datatype.

let len = (arr) {
	let index = 0;

	while true {
		try {
			let _ = arr[index];
		} catch {
			break;
		}

		index = index + 1;
	}

	return index;
};

datatype Iterator {
	let array = 0;
	let index = -1;

	let next = (self) {
		self.index = self.index + 1;

		if self.index < std.iter.len(self.array) {
			return self.array[self.index];
		}
	};
}

let new_iterator = (arr) {
	let iter = Iterator();
	iter.array = arr;
	return iter;
};
