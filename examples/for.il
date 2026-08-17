# This file defines a Counter datatype, which is used to demonstrate a for loop.
# For loops only continue if the condition, in this case `c.increment(42)`, returns something.
# Here, we can see that `increment()` only returns a value when the count isn't yet at the target.

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
