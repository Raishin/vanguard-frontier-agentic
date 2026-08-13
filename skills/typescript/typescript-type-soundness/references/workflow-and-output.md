# Workflow And Output

Diagnostic sequence and output contract for type-soundness review.

## Workflow

1. Identify every exported or shared type abstraction in scope: predicates, generics, conditional and mapped types, branded types.
2. For each type predicate, compare its runtime check against every property the narrowed type claims.
3. For each generic, determine its actual variance from its declared syntax and confirm the usage matches it.
4. Trace every `satisfies` usage and every escape hatch (`as`, `any`, `!`, `@ts-ignore`/`@ts-expect-error`) and classify each as justified or laundering.
5. Confirm the artifact under review is shared or published code, not a frontend application diff, before issuing a verdict.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the tsconfig strictness posture assumed.
- Variance, predicate/narrowing, `satisfies`/branded, and escape-hatch findings.
- A severity-labelled finding list, each with an evidence-basis label, and safe next actions plus any strictness assumption the user must confirm.
