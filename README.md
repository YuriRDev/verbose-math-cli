# ➕ Verbose-Math-Interpreter
 
Made this to practice a Rust and interpreters. *Probably gonna update this regularly, there's a few things that I'd like to change*  

---

### Usage

We don't want to make things simple, so ```1+1 * 5``` it's sooo 2020, now, it's ```1 plus 1 multiply 5```

```text
$ cargo run "5 plus 10 divide 2"
```
You don't need to specify the query on the params, you can simply run and then type.
```text
cargo run
5 plus 10 divide 2
7 multiply 5 multiply 2 minus 85 plus 99
...
```

| Meaning   | Lexeme       |                                    |
| :---------- | :--------- | :------------------------------------------ |
| `+`      | `plus` |  |
| `-`      | `minus` |  |
| `/`      | `divide` |  |
| `*`      | `multiply` |  |
| `^`      | `power` | _not implemented yet_ | 
| `(`      | `open_bracket` | _not implemented yet_ |
| `)`      | `close_bracket` | _not implemented yet_ |




---
### To-Do 
- [ ] AST It's not in it's best form (There's a few bugs to fix)
- [ ] Add unary-expressions (It's only binary for now)
- [ ] Add power operator, and brackets 
