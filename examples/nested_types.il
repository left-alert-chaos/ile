datatype Inner {
	let attr = 6;
}

datatype Middle {
	let attr = Inner();
}

datatype Outer {
	let attr = Middle();
}

let o = Outer();
let x = o.attr.attr.attr;
