import "tests/torture/primes.il";
import "tests/torture/info.il";
import "std";
let len = std.iter.len;

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

# check the value of an index
if info.array[2] != 3 {
	raise("Bad array indexing");
}

# cast an int to a float
if std.cast.float(5) != 5.0 {
	raise("Bad float casting");
}

# cast a float to an int
if std.cast.integer(5.0) != 5 {
	raise("Bad int casting");
}

# get the length of an array
let array = [1, 2, 3, 4];
if std.iter.len(array) != 4 {
	raise("Bad array counting");
}

# iterate over an array
let array2 = [];
let iterator = std.iter.new_iterator(array);
for let item = iterator.next(); {
	println(format("Found item ", item));
	array2 = array2 + [item];
}
if array2[3] != 4 {
	raise("Bad array iteration!");
}
