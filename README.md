# Mini-PL Interpreter
Compilers course project. Mini-PL is a simple programming language designed for learning purposes.
It contains a few statements, arithmetic expressions, and some IO primitives. The language is statically typed and has three built-in types: `int`, `string`, and `bool`.

## Sample program
```
var X : int := 1 + (2 * 6);
assert X < 11;

var n : int := 0;
print "How many times?";
read n;
var x : int;
for x in 0..n do
    print "Hello, World!\n";
end for;
assert (x = n);
```