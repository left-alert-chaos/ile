# Memory management
In Ile, memory management is super simple: when a variable is assigned to a value, that value is cloned, and when a variable goes out of scope, it is permanently removed. Ile has no references, pointers, or other way of holding multiple handles to the same object. This negates the need for any heavy memory regime. Here's a simple example to illustrate this point:

```
let function = () {
    let x = 1;
    let y = x;
    y = y + 1; # x doesn't update with y
}; # after here, both x and y are dropped

function();
```

Here, both `x` and `y` are defined in `function`'s scope. When `y` is created, `x`'s value is cloned, making them 2 distinct objects in memory. Therefore, when we then modify `y`, `x` stays the same.
