# sky-sl v0.1.0

## discussion

## types

### integer data types

| syntax | name                            |
| ------ | ------------------------------- |
| `u8`   | byte, or unsigned 8-bit integer |
| `u16`  | 16 bit unsigned integer         |
| `u32`  | 32 bit unsigned integer         |
| `u64`  | 64 bit unsigned integer         |
| `i8`   | 8 bit signed integer            |
| `i16`  | 16 bit signed integer           |
| `i32`  | 32 bit signed integer           |
| `i64`  | 64 bit signed integer           |

### floating point data types

| syntax | name                           |
| ------ | ------------------------------ |
| `f8`   | 8 bit unsigned floating point  |
| `f16`  | 16 bit unsigned floating point |
| `f32`  | 32 bit unsigned floating point |
| `f64`  | 64 bit unsigned floating point |

### types with special meaning

| syntax | name        | meaning                              |
| ------ | ----------- | ------------------------------------ |
| `()`   | empty tuple | similar to `void` in other languages |

## statements

| syntax             | statement            |
| ------------------ | -------------------- |
| `let name = expr;` | let statement        |
| `expr`             | expression statement |

## expression

| snytax               | expression                |
| -------------------- | ------------------------- |
| `loop {}`            | unbounded loop expression |
| `while cond {}`      | predicate loop expression |
| `for i in expr {}`   | iterator loop expression  |
| `if cond {} else {}` | if expression             |

## operator

### binary operator

| syntax | operator            |
| ------ | ------------------- |
| `+`    | Plus                |
| `-`    | Minus               |
| `*`    | Multiply            |
| `/`    | Divide              |
| `==`   | Equals              |
| `!=`   | NotEquals           |
| `>`    | GreatherThan        |
| `>=`   | GreatherOrEqualThan |
| `>>`   | Right shift         |
| `<`    | LesslThan           |
| `<=`   | LessOrEqualThan     |
| `<<`   | Left shift          |
| `%`    | Remainder           |
| `&`    | Bitwise And         |
| `|`    | Bitwise Or          |
| `^`    | Bitwise XOr         |
| `**`   | Exponent            |

### unary operator

| syntax | operator    |
| ------ | ----------- |
| `+`    | Keep Sign   |
| `-`    | Flip Sign   |
| `!`    | Boolean Not |
| `~`    | Bitwise Not |
