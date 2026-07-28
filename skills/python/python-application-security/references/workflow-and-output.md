# Review Workflow And Output Contract

The untrusted-input-to-sink review workflow and the required output shape.

## Workflow

1. Enumerate the trust boundaries: which inputs (request bodies, query params, headers, cookies, files, message payloads, env-influenced values) are attacker-controlled.
2. Trace each untrusted input to a sink: deserialization (`pickle`/`yaml.load`), dynamic execution (`eval`/`exec`), subprocess/shell, outbound request, or filesystem path.
3. For each reached sink, confirm whether a sound control exists (safe format, argument list, allowlist, path containment) and classify the CWE and severity.
4. Scan for secrets in source, logs, and exception paths, and for misused cryptography (weak hash, ECB, static IV, non-constant-time comparison).
5. Check security-critical error handling fails closed, and record every runtime/exploitability claim that needs out-of-band confirmation.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the trust boundary assumed (which inputs are attacker-controlled).
- The untrusted-input-to-sink findings for deserialization, dynamic execution, injection, SSRF, path/file handling, secrets, and cryptography.
- A severity-labelled finding list, each with an evidence-basis label and CWE where applicable, plus safe remediations and any exploitability claim the user must confirm.
