# Objects and classifications
In most programming languages, objects or values have types. This is not true in Ile. Instead, Ile opts for a similar-but-different system: _classifications_. Because Ile is so tightly bound to Rust, the classification of an object is what determines what Rust type it wraps. Ile uses a dynamically-typed system; variables can be re-assigned to an object of any classification. However, some operations like math or comparisons require objects to be the same classification.

Here's a list of all classifications:

- Function
- Data
- Integer
- Float
- Boolean
- String
- Array

As you can see, classifications serve to separate different primitives. What's really important to talk about now, though, is Data.

## Data
In Ile, a Data object is an object that, instead of wrapping one singluar Rust value, wraps a `HashMap` of attributes. These attributes can be any other object.

What this means in English is that Data objects are the only objects that can have attributes, or hold other objects.

### Datatypes
To create a piece of Data, you need a `datatype`. A `datatype` is like a template of what attributes a piece of Data has. They don't do anything else and, once created, Data objects don't store what `datatype` was used to create them. Let's look at an example:

```
datatype MyType {
    let attribute1 = 1;
    let attribute2 = "string";
}

let instance = MyType();
```

`datatype`s are a list of variable initializations in their own block. Only `let` statements are allowed inside `datatype`s.

You can instantiate the `datatype` by calling it like a function without arguments. In the above example, the `instance` variable holds a Data object with 2 attributes: `attribute1` and `attribute2`.

You can access and re-assign attributes the same way you can in many other languages. If you wanted to print out the value of `attribute1`, you would write:

```
println(instance.attribute1);
```

## Functions
There's one more object classification we need to talk about: functions. Ile doesn't have a special keyword for functions, and in fact treats them much like any other object. You even define them with a `let` statement! Here's an example of a function:

```
let my_func = () {
    println("my_func is running!");
};
```

Here, we create a variable with the name `my_func` and set it its value to a function. Functions are defined as `(<arguments>) {code}`. If our function took arguments, they would be between the parentheses. Because Ile is dynamically-typed, there is no typing syntax. Before we move on, note how there is a semicolon at the end of the definition to end the `let` statement.

### Methods
Some functions are special in that they belong to a Data object. These functions are called methods, and act slightly differently. Here's an example with our previous `datatype`.

```
datatype MyType {
    let attribute1 = 1;
    let attribute2 = "string";

    let method1 = (self) {
        println(self.attribute2);
    };
}

let instance = MyType();
instance.method1();
```

In this example, we define a function inside a `datatype`, and give it a `self` argument. If the first argument of a function is called `self`, its value is automatically set to that of the object whose method is being called. After the function completes, the value of `self` is read again, and the original variable's value is set to `self`'s value. All this means in practice is that you can re-assign attributes inside of methods and the object will automatically update after the method completes.
