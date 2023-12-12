# Simulador SISA

The types are self explanatory, just mess around with `main` ig.

The code is incredibly self documenting (on purpose).

I also use `transmute` a lot to interpret the bits into the way I want. Such is the way
of low level code.

# NOTE
It assumes the input is wellformed. Do not feed it instuctions like

`MOVI R5, 0x555`

because it's invalid. If you feed it invalid input, you're gonna get UB (good luck lmao).

# IO
Use `[cpu].update_io(new_io)` to change the IO status in between `execute`s.
