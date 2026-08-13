# Cost Model Formulas

Each calculation written out with its units and its sensitivity variables.

- Annual engineering-hours lost is computed from a supplied per-incident or per-wait-event time cost multiplied by supplied frequency, converted to an annual figure with units stated as hours per year, never presented as a bare number.
- CI compute cost is computed from a supplied per-run duration and per-unit compute cost multiplied by supplied run frequency, with the distinction between a mean and a median duration carried into the result rather than discarded.
- Break-even is the point at which cumulative cost of the status quo equals cumulative cost of the investment (migration cost plus any new steady-state cost) over time, expressed in the same units as the underlying inputs, typically calendar time or engineering-hours.
- Cost of postponement is the marginal cost of the status quo per additional period of delay, distinct from break-even, and uses the same supplied inputs so the two figures stay internally consistent.
- Sensitivity analysis identifies which single input, if varied within a plausible supplied range, moves the break-even or postponement conclusion the most, and states the threshold value at which the recommendation would flip.
- A calculation with no stated units, or one that mixes units (for example hours in one term and dollars in another without an explicit loaded-cost conversion), is not a valid output regardless of whether the arithmetic is otherwise correct.
