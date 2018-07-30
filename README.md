# Schematic

Schematic is a Lisp-like language implemented in Rust.

It's intended for two purposes:
1. Teach me Rust
2. Be a playground for programming language concepts

...but right now it just parses atoms and [S-expressions](https://en.wikipedia.org/wiki/S-expression) :)

## Running Schematic

Setting the `DEBUG` environment variable to `true` will print extra information.

```bash
Debug mode enabled.
λ (foo bar baz)
 DEBUG: raw input = (foo bar baz)
 DEBUG
  ;;; val begins here
ValuePtr {
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
  ;;; val ends here
 DEBUG

 ==> (foo bar baz)
λ 100
 DEBUG: raw input = 100
 DEBUG
  ;;; val begins here
ValuePtr {
    obj: Atom(
        Fixnum(
            Fixnum {
                value: 100
            }
        )
    )
}
  ;;; val ends here
 DEBUG

 ==> 100
λ "Hello!"
 DEBUG: raw input = "Hello!"
 DEBUG
  ;;; val begins here
ValuePtr {
    obj: Atom(
        String(
            Str {
                value: "\"Hello!\""
            }
        )
    )
}
  ;;; val ends here
 DEBUG

 ==> "Hello!"
λ +
 DEBUG: raw input = +
 DEBUG
  ;;; val begins here
ValuePtr {
    obj: Atom(
        Symbol(
            Symbol {
                value: "+"
            }
        )
    )
}
  ;;; val ends here
 DEBUG

 ==> +
λ true
 DEBUG: raw input = true
 DEBUG
  ;;; val begins here
ValuePtr {
    obj: Atom(
        Boolean(
            Boolean {
                value: true
            }
        )
    )
}
  ;;; val ends here
 DEBUG

 ==> true
λ 
Goodbye.
```
