# Workflow And Output

Diagnostic sequence and output contract for MCP tool-contract review.

## Workflow

1. Identify the MCP specification revision and SDK generation the code targets.
2. Compare each tool's `inputSchema`/`outputSchema` against the handler's actual accepted input and returned output.
3. Check `structuredContent` responses validate against their declared `outputSchema`.
4. Check protocol-version negotiation, `server/discover`, and error-channel classification against the current specification.
5. Check the tool registration surface for completeness and any model-facing text that could act as an injection vector.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the MCP specification revision / SDK generation assumed.
- Schema-fidelity, structured-output, protocol-version/error-contract, registration-surface, and SDK-generation findings, each with an evidence-basis label.
- A severity-labelled finding list plus safe next actions and open questions, including anything the security board or a vendor-connector agent must confirm.
