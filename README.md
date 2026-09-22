# func-plot

A small Rust function plotter. Each expression is drawn in a different color on shared axes.

Start the program, then enter one expression per line in the same terminal. The first valid
expression opens the plot window; later expressions are added to it immediately.

```sh
cargo run
f(x)> sin(x)
f(x)> cos(x)
f(x)> -x^2/10
```

Type `:quit` in the terminal or close the window to exit. Invalid expressions are reported
without removing curves already on the plot.

The range is `min;max;step` and defaults to `-10;10;0.02`. Set it at startup if needed:

```sh
cargo run -- --range "-5;5;0.01"
```

You can also pass initial expressions as arguments, then keep adding expressions in the terminal:

```sh
cargo run -- "x^2" "-x+2" --range "-5;5;0.01"
```

Quote expressions passed as arguments so the shell does not interpret operators or parentheses.
Supported operators are `+`, `-`, `*`, `/`, and `^`; signs can be unary, as in `-x^2`
or `2^-3`. Power binds more tightly than unary minus, so `-x^2` means `-(x^2)`. Functions
are `sin(...)`, `cos(...)`, `sqrt(...)`, and `log(...)` (natural logarithm). Constants are
`pi` and `e`. Multiplication must be explicit: use `2*x`, not `2x`.

The plot skips undefined values and breaks lines across large jumps.
