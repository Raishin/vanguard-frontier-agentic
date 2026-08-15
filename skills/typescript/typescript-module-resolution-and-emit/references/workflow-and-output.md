# Workflow And Output

Diagnostic sequence and output contract for module-resolution-and-emit review.

## Workflow

1. Read `package.json` and every `tsconfig.json`, and establish the declared consumer list.
2. Check the `module`/`moduleResolution` values against the installed compiler's actually-accepted set.
3. Check `exports`/`imports` condition ordering, confirming `types` is first and `default` is last.
4. Trace `.mts`/`.cts` usage and confirm it matches the intended module format per file.
5. Confirm the consumer matrix claimed (Node ESM, Node CJS, bundler modes, test runner) has actual supporting evidence, not assumption.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the consumer matrix assumed.
- `module`/`moduleResolution`, `exports` ordering, `.mts`/`.cts`, and dual-package findings.
- A severity-labelled finding list, each with an evidence-basis label, and safe next actions plus any consumer mode the user must confirm is in scope.
