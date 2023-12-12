# Simulador SISA

The types are self explanatory, just mess around with `main` ig.

The code is incredibly self documenting (on purpose).

I also use `transmute` a lot to interpret the bits into the way I want. Such is the way
of low level code.

## IO
Use `[cpu].update_io(new_io)` to change the IO status in between `execute`s.

## Branching
They're implemented but untested because I am lazy

## NOTE
It assumes the input is wellformed. Do not feed it instuctions like

`MOVI R5, 0x555`

because it's invalid. If you feed it invalid input, you're gonna get UB (good luck lmao).

Also, the memory and data memories are separate because I didn't stop to think before I started 
writing down code. Just, like, assume they're the same. I'm protecting you from yourself?
