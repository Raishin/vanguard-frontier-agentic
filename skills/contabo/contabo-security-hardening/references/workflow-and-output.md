# Workflow and output contract

Use this reference only when performing a full security posture review, generating hardening recommendations, or producing a structured audit report for a Contabo environment.

## Review domains

Check these areas before giving a verdict:

- SSH key strategy: Contabo secret IDs used vs. raw private key material in scripts or API payloads
- Default user policy: root login disabled, admin user configured via Cloud-Init, password login disabled
- Firewall posture: open inbound ports, 0.0.0.0/0 rules, management port exposure
- OAuth2 credential hygiene: token TTL awareness (~5 min), environment variable storage, absence of logged token values
- API traceability: x-request-id (UUIDv4) included in all REST calls
- Secret scanning: hardcoded credentials in automation scripts, CI/CD pipelines, or userData
- Network isolation: Private Networking add-on usage, Additional IP exposure, unnecessary public-facing services

## Safe workflow

1. **Frame scope**
   - Instance(s) or environment under review:
   - Business criticality and owner:
   - Data classification or compliance driver:
   - Required outcome (gap report, remediation plan, hardening script):
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer user-provided sanitized configuration evidence (firewall rules, sshd_config, Cloud-Init fragments).
   - Use read-only Contabo API calls if live access is available.
   - Otherwise ground in official Contabo docs and label accordingly.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What SSH or authentication path can expose root access?
   - What firewall rule allows lateral movement or unrestricted inbound?
   - What credentials or secrets are hardcoded or logged?
   - What OAuth2 token handling creates a replay or leakage window?
   - What evidence is missing that would change the risk assessment?
4. **Recommend the smallest safe hardening action**
   - Prefer targeted, reversible changes with a validation step.
   - If the safest action is to gather evidence before recommending changes, say that plainly.
   - Never suggest disabling the only SSH access path without confirming an alternative.

## Output contract

Return this structure:

```markdown
# Contabo Security Hardening Review: <scope>
## Executive verdict
- Status: HARDENED / HARDENED WITH GAPS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
