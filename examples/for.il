datatype Counter {
	let count = 0;
	let increment = (self, target) {
		self.count = self.count + 1;
		if self.count < target {
			return self.count;
		}
	};
}

let c = Counter();

for c.increment(42) {}

let x = c.count;
