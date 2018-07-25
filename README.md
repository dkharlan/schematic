# Schematic

Schematic is a Lisp-like language implemented in Rust.

It's intended for two purposes:
1. Teach me Rust
2. Be a playground for programming language concepts

...but right now it just parses atoms and [S-expressions](https://en.wikipedia.org/wiki/S-expression) :)

## Running Schematic

```bash
$ cargo run
     (...Cargo output...)
λ (foo bar baz)
 DEBUG: raw input = (foo bar baz)
 ==> ValuePtr {
    obj: Cons(
        Cons {
            left: Atom(
                Symbol(
                    Symbol {
                        value: "foo"
                    }
                )
            ),
            right: Cons(
                Cons {
                    left: Atom(
                        Symbol(
                            Symbol {
                                value: "bar"
                            }
                        )
                    ),
                    right: Cons(
                        Cons {
                            left: Atom(
                                Symbol(
                                    Symbol {
                                        value: "baz"
                                    }
                                )
                            ),
                            right: Nil
                        }
                    )
                }
            )
        }
    )
}
λ 100
 DEBUG: raw input = 100
 ==> ValuePtr {
    obj: Atom(
        Fixnum(
            Fixnum {
                value: 100
            }
        )
    )
}
λ "Hello!"
 DEBUG: raw input = "Hello!"
 ==> ValuePtr {
    obj: Atom(
        String(
            Str {
                value: "\"Hello!\""
            }
        )
    )
}
λ +
 DEBUG: raw input = +
 ==> ValuePtr {
    obj: Atom(
        Symbol(
            Symbol {
                value: "+"
            }
        )
    )
}
λ true 
 DEBUG: raw input = true
 ==> ValuePtr {
    obj: Atom(
        Boolean(
            Boolean {
                value: true
            }
        )
    )
}
λ ^D
Goodbye.
$
```
