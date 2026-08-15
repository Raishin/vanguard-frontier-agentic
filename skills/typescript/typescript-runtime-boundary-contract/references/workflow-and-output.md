# Workflow And Output

Diagnostic sequence and output contract for boundary-contract review.

## Workflow

1. Enumerate every boundary the code can be entered through (HTTP, queue, webhook, environment/config, database, third-party SDK, file, agent/tool call).
2. Trace each boundary to its own parse call and confirm no alternate path bypasses it.
3. Confirm the schema and the TypeScript type share one source of truth rather than being separately maintained.
4. Check every result-returning parse call (`safeParse` and equivalents) branches on failure and short-circuits.
5. Confirm generated schemas or types show evidence of being regenerated alongside the definitions they mirror.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the boundary inventory assumed complete.
- Parse-versus-assert, `unknown`-first, schema/type single-source-of-truth, and drift findings.
- A severity-labelled finding list, each with an evidence-basis label, and safe next actions plus any boundary the user must confirm is covered.
