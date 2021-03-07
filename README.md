# Chorus-lang syntax

## Developers note

This is my first language please don't booli me

## Keywords

- `let`
- `fn`
- `while`
- `for`
- `if`

## Operators

|  Name  |             Definition             |
|--------|------------------------------------|
| `//`   | Comment                            |
| `+`    | Plus  (add)                        |
| `-`    | Minus (subtract)                   |
| `*`    | Star  (multiply)                   |
| `/`    | Slash (divide)                     |
| `=`    | Assign                             |
| `<`    | Less than                          |
| `>`    | Greater than                       |
| `==`   | Equal                              |
| `<=`   | Less than/eq                       |
| `>=`   | Greater than/eq                    |
| `\|`   | Or (bitwise)                       |
| `\|\|` | Or (normal)                        |
| `&`    | And (bitwise)                      |
| `&&`   | And (normal)                       |
| `\|>`  | Pipe (refer to ecma documentation) |

## Examples

- Let

```_
let foo<Type> = bar;
```

- Function

```_
fn foo<Generic T>(, arg<Type>) => <Type> { ... }
```
