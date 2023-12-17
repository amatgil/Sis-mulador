# Simulador SISA

The code is incredibly self documenting (on purpose). Usage is below.

(the ergonomics aren't the best, but it works Well Enough)

## Example usage
To solve the alien that I had for 13c (which gave instuctions and initial memory, with an initial
PC of 0 and no initial register state), use:
```rs
cargo run -- examples/alien/alien.sisa -m examples/alien/alien.smem -i examples/alien/alien.sio
```

For the `mul16` algorithm:
```rs
cargo run -- examples/mul16/mul16.sisa -r examples/mul16/mul16.sregs
```

Or for the `mulfast` algorithm:
```rs
cargo run -- examples/mulfast/mulfast.sisa -r examples/mulfast/mulfast.sregs
```

Both multiplication algorithms will multiply 5 times 10, giving `0x32` on `R5`.


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

You cannot use more than eight registers, it will panic (this is intended behavior).

## NOTE
It assumes the input is wellformed. Do not feed it instuctions like

`MOVI R5, 0x555`

because it's invalid. If you feed it invalid input, you're gonna get UB (good luck lmao).

Also, the memory and data memories are separate because I didn't stop to think before I started 
writing down code. Just, like, assume they're the same. I'm protecting you from yourself (this actually
has already helped me).

## Roadmap
- Add a preprocessor for directives, labels, etc.
- Use `as` more (note the sign extension remarks when upcasting): https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
- Implement `JALR` when we know more about it
