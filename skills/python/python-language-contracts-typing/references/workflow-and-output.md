# Review Workflow And Output Contract

The type-contract review workflow and the required output shape.

## Workflow

1. Identify the public boundaries (exported functions, class APIs, module interfaces) and the type-checker configuration assumed.
2. Trace `Any` (explicit and implicit) across those boundaries and flag every leak that erases caller safety.
3. Check Protocol usage, generic variance soundness, and overload/implementation consistency.
4. Check TypedDict/dataclass contracts (required keys, mutable defaults) and that trust boundaries carry runtime validation, not annotations alone.
5. Record every claim that depends on the checker's actual output or configuration as needing the user's confirmation.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the type-checker/strictness assumed.
- Any-propagation, Protocol/generic/variance, overload/TypedDict/dataclass, and runtime-validation findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any checker configuration the user must confirm.
