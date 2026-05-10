## EVAL DEFINITION: pr18-deep-security-audit

**Scope:** Deep security audit of PR diffs (claude/add-eu-cloud-providers-6NGhv vs origin/master).
**Reviewer stance:** Senior security auditor / pentester, Fortune-50 production experience.
**Method:** Diff-only review (added/removed lines), not whole files. Focus on uncommon attack vectors, OWASP, regressions.

### Capability Evals (CE)

CE-1: **Secret leakage** — no API keys, OAuth tokens, customer/tenant/account IDs, JWTs, or PII present in any added line.
CE-2: **Privilege correctness** — every `allowed-tools` and `sandbox_mode` declaration in the diff matches the agent/skill's stated capability scope. No silent privilege escalation.
CE-3: **Injection surfaces** — every `Bash`/`curl`/`jq`/`uuidgen` snippet added in the diff is robust against:
  - Command injection via env var or user-supplied parameter
  - SSRF via attacker-controlled hostname or path
  - JSON / shell metacharacter injection
  - URL parsing pitfalls (auth-mismatch, redirect, port-spoofing)
  - Token logging / leakage
CE-4: **Approval-gate integrity** — hard-stop conditions in added live-guard agents/skills cannot be bypassed by:
  - Ambiguous identity strings
  - Missing or stale evidence labels
  - Race conditions between read-only inventory and mutation
  - Confused-deputy / IDOR (target ID supplied without ownership proof)
CE-5: **Supply-chain & file-system risk** — no symlinks, no executable artifacts, no `eval`-style or `curl | sh` patterns, no SVG with embedded scripts/foreignObject/xlink.
CE-6: **Validator/schema regressions** — schema enum + catalog allowlist additions are scoped to the 5 EU providers, do not loosen any existing validation, and `validate-catalog.py` still rejects secret patterns.
CE-7: **Doc-grounded safety** — README/AGENT/SKILL prose does not advertise capabilities the implementation cannot enforce (false-confidence regression).
CE-8: **OWASP Top-10 / LLM-Top-10 coverage** — review for prompt injection, insecure output handling, sensitive info disclosure, broken access control, SSRF, vulnerable components.

### Regression Evals (RE)

RE-1: All 7 validation gates still pass after fixes.
RE-2: No reduction of guard rails on previously existing AWS/Azure/OCI agents.
RE-3: Schema validation still rejects unknown providers and malformed metadata.

### Severity scale

- CRITICAL: exploitable remote, secret leak, RCE, IDOR with mutation
- HIGH: privilege escalation, bypass of hard-stop, plaintext secret persistence
- MEDIUM: weak gate semantics, ambiguous capability contract, doc/impl drift
- LOW: documentation hygiene, log discipline, naming inconsistency
- INFO: hardening suggestion

### Pass criteria

- pass^3 = 1.00 for RE-1..RE-3 (release gates)
- pass@3 ≥ 0.90 for CE-1..CE-8 (capability evals)
- Zero CRITICAL findings to merge
