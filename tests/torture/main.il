# do some math
let x = 5 + 2 * 3;
println(format("x is ", x));
if x != 11 {
	raise("Failed to calculate");
}

# concatenate some strings
if "string1 " + "string2" != "string1 string2" {
	raise("Failed to concatenate 2 strings");
}
