# Money, Rounding, And Decimal

Why float is wrong for money and how Decimal and explicit rounding fix it.

- Binary floating point cannot represent most decimal fractions exactly (the documented `0.1` example), so accumulating money in `float` produces representation error that compounds across operations.
- `decimal.Decimal` provides exact decimal arithmetic with a configurable context; a `Decimal` must be constructed from a string or integer, because constructing it from a `float` inherits the float's imprecision.
- `Decimal.quantize(exp, rounding=ROUND_*)` applies an explicit rounding rule at a fixed number of places; Python's built-in `round` uses round-half-to-even, which is not the half-up rule many financial contexts assume.

## Sources

- https://docs.python.org/3/tutorial/floatingpoint.html
- https://docs.python.org/3/library/decimal.html
