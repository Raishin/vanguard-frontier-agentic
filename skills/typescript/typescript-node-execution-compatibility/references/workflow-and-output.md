# Workflow And Output

Diagnostic sequence and output contract for node-execution-compatibility review.

## Workflow

1. Establish the exact run command, flags, and target Node version — refuse-and-ask if any is missing.
2. Check the executed source for constructs that throw under Node's type stripper (`enum`, runtime `namespace`, parameter properties, `import =`, decorators).
3. Confirm a separate `tsc --noEmit` (or equivalent) gate exists in CI, distinct from the production execution path.
4. Check for `paths`-alias reliance and extension-less imports in code intended for direct execution.
5. Confirm every capability claim (stripping default/stable status, a CLI flag) is scoped to the confirmed Node version against the current release line.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the target Node version assumed.
- Type-stripping/unsupported-syntax, separate-typecheck-gate, `paths`/import-extension, and version-gating findings.
- A severity-labelled finding list, each with an evidence-basis label, and safe next actions plus any Node version or run command the user must confirm.
