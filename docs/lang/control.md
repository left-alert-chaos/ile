# Control flow
Like many other languages, Ile provides a few different methods for control flow. I'll run through all of them quickly here.

## Functions
As already discussed, functions can be methods or normal, stand-alone blocks. Here's an example:

```
let my_func = () {
    println("my_func was executed!");
};
```

## Loops

### For loops
In Ile, `for` loops act differently than in other languages. Where other languages will automatically iterate for you, Ile takes a simpler aproach: a `for` loop is made up of two sections, the condition and the body. For one loop iteration: the condition is executed. If it returns _anything_, the body is executed. Otherwise, the loop breaks.

Here's an example:

```
use "std";
let iterator = std.iter.new_iterator([1, 2, 3]);
for let item = iterator.next(); {
    println(format("iterator found item ", item));
}
```

### While loops
Where a `for` loop will keep executing until its condition returns nothing, a `while` loop will keep executing until its condition returns `false`. The loop will even keep going if the condition returns nothing!

Here's an example:

```
while true {
    println("hihihihihihihi");
}
```

### `break` and `continue`
If you've programmed before, you know what these do: `break` ends a loop on the stop and `continue` skips the rest of the loop and starts over.

Both expect a semicolon.

Here's an example of `break` that does nothing:

```
while true {
    break;
}
```

And here's an example of `continue` that loops infinitely _without printing_:

```
while true {
    continue;
    println("This shouldn't show up!");
}
```

## `try` and `catch`
Ile does error handling through `try` and `catch` blocks. If the code block after the `try` keyword raises an error, it stops and the `catch` block executes without crashing.

Here's an example:

```
try {
    raise("Oh, no! An error!");
} catch {
    println("Error successfully caught without crashing.");
}
```
