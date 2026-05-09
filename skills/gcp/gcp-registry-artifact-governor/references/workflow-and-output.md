# Workflow and output contract

Use this reference only when performing the full registry governance review, supply chain posture assessment, or hardening implementation pass.

## Review domains

Check these areas before giving a verdict:
- Repository IAM: allUsers/allAuthenticatedUsers bindings, least-privilege service account assignments, cross-project access paths
- Binary Authorization: enforcement mode (Allow all / Allowlist / Require attestation), attestor list, GKE cluster binding, break-glass policy
- Container Analysis: automatic scanning on push status, severity threshold configuration, attestor integration
- Retention policies: cleanup policy existence, tag-based retention rules, untagged digest accumulation risk
- CMEK: encryption key configuration for regulated repositories
- CI/CD access: Workload Identity Federation vs. service account key usage
- Cross-project access: explicit repository-level IAM binding vs. assumed project-level cascade

## Safe workflow

1. **Frame scope**
   - Registry project and repository list:
   - GKE clusters consuming the registry:
   - Compliance requirements (CMEK, data residency):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test supply chain posture**
   - Is Binary Authorization in "Require attestation" mode or weaker?
   - Are there any allUsers or allAuthenticatedUsers IAM bindings on repositories?
   - Are vulnerability severity thresholds enforced by attestors or advisory only?
   - Do untagged digests accumulate without a cleanup policy?
   - What evidence is missing?
4. **Recommend the smallest safe hardening action**
   - Prefer narrow scope, staged enforcement (Binary Authorization dry-run first), validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Registry Artifact Governor: <scope>
## Executive verdict
- Status: SECURE / SECURE WITH GAPS / EXPOSED / NEEDS EVIDENCE
- Supply chain enforcement: <Binary Authorization mode>
- Biggest gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended hardening actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Retention and storage hygiene
- Untagged digest accumulation: <present / absent / unknown>
- Cleanup policy: <configured / missing / unknown>
## Residual risk
- <risk or explicit none>
```
