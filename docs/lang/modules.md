# Modules
Like many other languages, Ile uses modules to separate code into multiple files. This makes maintenance easier because it's easier to find the logic you're looking for. To add another module to the scope, you use the `import` keyword. You use it to import both other Ile source files and your runtime's libraries. The name of the module you're importing should be in quotes, like so:

```
import "std";
```

To import another Ile file, you use the path to the file from your project root. For example, that could look something like this:

```
import "directory1/directory2/module.il";
```

Because modules can import each other, you can also create files that exist just to bundle multiple modules together.
