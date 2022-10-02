# Expressions

The expression system in shork is pretty "boring". It is similair to C, except that bitwise operations are evaluated before equality and after comparison. Below, you can see the expression grammar:

```txt
expr                -> equality
equality            -> comparison ( ("!="|"==") comparison )*
comparison          -> bitwise ( (">"|">="|"<"|"<=") bitwise )*
bitwise             -> term ( ("|"|"&"|"<<"|">>") term)*
term                -> factor ( ("-"|"+") factor)*
factor              -> unary ( ("/"|"*"|"%") unary)*
unary               -> ("!"|"-") unary | primary
primary             -> NUMBER | STRING | REGEX | STRUCT | "true" | "false" | "(" expr ")"
```

## Supported Operators

| Operator | Description                                                                                                                                   |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| !=       | Tests if two expressions aren't equal                                                                                                         |
| ==       | Tests if two expressions are equal                                                                                                            |
| >        | Tests if the left side of the operator is greater than the right side                                                                         |
| >=       | Tests if the left side of the operator is greater or equal than the right side                                                                |
| <        | Tests if the left side of the operator is lesser than the right side                                                                          |
| <=       | Tests if the left side of the operator is lesser or equal than the right side                                                                 |
| \|       | Performs the bitwise OR operation                                                                                                             |
| &        | Performs the bitwise AND operation                                                                                                            |
| <<       | Shifts the left side of the operator to the right (as many bit as the right side of the operator)                                             |
| >>       | Shifts the left side of the operator to the right (as many bit as the right side of the operator)                                             |
| -        | Subtracts the right side of the operator from the left side of the operator, or makes the right side negative (when used as a unary operator) |
| +        | Adds the right side of the operator to the left side of the operator (concatenates strings)                                                   |
| /        | Divides the left side of the operator by the right side of the operator                                                                       |
| *        | Multiplies the left side of the operator by the right side of the operator                                                                    |
| %        | Performs the modulo operation                                                                                                                 |
| !        | The logical NOT                                                                                                                               |
