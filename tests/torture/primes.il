# This file is used to test calculation of prime numbers

import "std";

# find the first 1000 primes
let fifty_primes = () {
	let primes = [];
	
	let num = 1;
	let num_of_primes = 0;

	while num_of_primes < 50 {
		num = num + 1;

		let primes_iterator = std.iter.new_iterator(primes);
		let is_prime = true;

		for let prime = primes_iterator.next(); {
			if is_divisible(num, prime) {
				is_prime = false;
				break;
			}
		}

		if is_prime {
			primes = primes + [num];
			num_of_primes = num_of_primes + 1;
			println(format("found prime ", num_of_primes, ": ", num));
		}
	}
	
	return primes;
};
