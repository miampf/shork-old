# Writing Functions

```Doesn't work rn```

Basically, to write a function, you just write ```define name()``` followed by curly parantheses.

Like this:

```txt
define name(){

}
```

You can give them parameters and return values, for example, a function that sums up two integers could look like this:

```txt
define sum(one: integer, two: integer): integer{
    return one + two
}
```

Generic types are also possible (well, not really, but they are similair). A generic is indicated with a "T" and have the variable types that they can represent written besides them in "<>".

The same function but with generics could be:

```txt
define sum(one: T<integer|float>, two: T<integer|float>): T<integer|float>{
    return one + two
}
```
