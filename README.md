# Mini-PL Interpreter
Compilers 2020 course project. Mini-PL is a simple programming language designed for learning purposes.
It contains a few statements, arithmetic expressions, and some IO primitives. The language is statically typed and has three built-in types: `int`, `string`, and `bool`.

## Sample program
```
var nTimes : int := 0;
print "How many times?";
read nTimes;
var x : int;
for x in 0..nTimes-1 do
    print x;
    print " : Hello, World!\n";
end for;
assert (x = nTimes);
```