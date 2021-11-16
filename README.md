
# printd

This crate provides a slightly modified version of the `dbg!()` macro.
The `printd!()` macro prints one or more expression which can be annotated with a label.

In contrast to the `dbg!()` macro from `std`, it does *not* use the "fancy" debug formatter (`:#?`)
as it's output is sometimes just to big if you debug more then a few variables.

The macro also supports printing debug messages and labeling multiple expressions. (See examples.)

There is also `eprintd!()` for printing to `stderr`.

## Examples

Basic usage.

```rust

printd!(1 + 2);
// [printd/README.md:20] 1 + 2 = 3

```

Usage like `dbg!()`
But way smaller output.

```rust

let foo = vec![1, 2, 3];
dbg!(foo);
// [printd/README.md:31] foo = [
//     1,
//     2,
//     3,
// ]

let foo = vec![1, 2, 3];
printd!(foo);
// [printd/README.md:39] foo = [1, 2, 3]

```

Supports multiple expressions.

```rust

let foo = vec![1, 2, 3];
let bar = 69;
printd!(foo, bar);
// [printd/README.md:50] foo = [1, 2, 3]
// [printd/README.md:50] bar = 69

```

Supports (pretty) debug messages.
The `dbg!()` macro don't cares if the message is a string literal and 
prints ugly output.

```rust

dbg!("Hello world!");
// [printd/README.md:62] "Hello world!" = "Hello world!"

printd!("Hello world!");
// [printd/README.md:65] "Hello world!"

```

You can label debug expressions to organize them a bit more.
Expressions with a label print the source location just once.

```rust

let foo = vec![1, 2, 3];
let bar = 69;
let idk = String::from("Something else then foo and bar\n");

dbg!("Very important variables", foo, bar);
// [printd/README.md:77] Very important variables
//     foo = [1, 2, 3]
//     bar = 69
//     idk = "Something else then foo and bar\n"

```
