import "tests/torture/primes.il";

# do some math
if 5 + 2 * 3 != 11 {
	raise("Failed to calculate");
}

# concatenate some strings
if "string1 " + "string2" != "string1 string2" {
	raise("Failed to concatenate 2 strings");
}

# format some text
if format("number is ", 5) != "number is 5" {
	raise("Failed to concatenate string and number");
}

# negative numbers
println(format("Negative one is ", -1));
if -1 >= 0 {
	raise("Negative one is greater than or equal to zero");
}

