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

Generic types are also possible. A generic is indicated with a "T". If you want that generic to have specific traits, you can add attributes inside of "<>" parantheses.

The same function but with generics could be:

```txt
define sum(one: T<number>, two: T<number>): T<number>{
    return one + two
}
```

Here, "number" is an attribute that all numbers have (integers, doubles, floats, longs, etc.). For a guide on attributes, look into [Attributes]()
