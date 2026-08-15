# Measurement Intake And Refusal

The required input list, the labelling scheme, and the refusal template naming what is missing.

- The required input set this skill accepts is user-supplied only: CI durations, developer headcount and loaded cost, local developer wait times, incident counts, support-ticket volume, and migration-effort estimates — nothing beyond what the user volunteers is fetched or inferred.
- Every value in an output carries exactly one label: measured (the user directly observed and reports the number), supplied (the user gave the number without describing how it was obtained), or assumed (this skill filled a gap) — an assumed label must name the specific assumption in the same sentence.
- The refusal template for a missing material input names the exact input missing, states what the requested figure cannot be computed without it, and states what evidence would resolve the gap — it never substitutes an industry-average or otherwise invented placeholder.
- A request framed as wanting a rough number or just a ballpark is treated identically to a missing-input request: this skill either asks for the input or explicitly declines to produce a figure.
- A migration-cost estimate is checked for whether it includes review and rollout cost, not only the mechanical code-change estimate, before it is accepted as a valid input to break-even or postponement.
- This skill's own acceptance is conditional and time-boxed: it must never be the first agent dispatched on a task, and it is re-prosecuted two quarters after shipping, removed if it produced no engineering decision in that window.
- The board maintainer who merges this agent owns the two-quarter re-prosecution obligation; omitting that review converts an explicitly conditional acceptance into a de facto permanent one.
