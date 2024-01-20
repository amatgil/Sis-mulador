# Simulador SISA

The code is incredibly self documenting (on purpose). Usage is below.

(the ergonomics aren't the best, but it works Well Enough)

## Example usage
To solve the alien that I had for Tema 14 
```rs
cargo run -- examples/complete/first_example.sisa
```

You may also specify some initial values for the registers and IO:

```rs
cargo run -- [code file] -r [registers file] -i [io file]
```


## IO
Use `[cpu].update_io(new_io)` to change the IO status in between `execute`s. This cannot be done
from the cli, at the moment, because I have no idea how (and it doesn't seem that useful, to be 
honest).

## Registers
File must contain eight lines (or less, for a computer with less registers), each with a decimal number. E.g.
```txt
0
0
0
0
0
0
5
17
```

Register `6` will hold `0x0005` and Register `7` will hold `0x0011`.

You cannot use more than eight registers, it will abort (this is intended behavior).

## NOTE
It assumes the input is wellformed. Do not feed it instuctions like

`MOVI R5, 0x555`

because that is invalid. If you feed it invalid input, you're gonna get UB (good luck lmao).

Also, the memory and data memories are separate because I didn't stop to think before I started 
writing down code. Just, like, assume they're the same. I'm protecting you from yourself (this actually
has already helped me).

## Ideas
- Use `as` more (note the sign extension remarks when upcasting): https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions


# Notice
- Don't `.set` words like verbs, register names or labels. That's UB.
- Don't mismatch parens. That's UB, though I try to catch them
- Jumps are always relative, so JALR may behave... unexpectedly (wrong). Careful!
- Don't try to make jumps too large. There's a check for this but it's untested
